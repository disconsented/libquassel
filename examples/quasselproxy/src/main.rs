use failure::Error;

use std::result::Result;
use std::vec::Vec;

use core::marker::Unpin;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};

use libquassel::{
    frame::QuasselCodec,
    message::{ConnAck, HandshakeMessage},
};

use log::{debug, error, info, trace};

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();

    let host = std::env::args().nth(1).expect("no host given");
    let host: Vec<&str> = host.split(':').collect();
    let username = std::env::args().nth(2).expect("no username given");
    let password = std::env::args().nth(3).expect("no password given");

    let server = Server::new(
        "test",
        ServerSettings {
            tls: false,
            compression: false,
            host: host[0].to_string(),
            port: host[1].parse().unwrap(),
            username,
            password,
        },
    );

    //
    // Start Server Connection
    //

    let mut s_server =
        TcpStream::connect(format!("{}:{}", server.settings.host, server.settings.port)).await?;

    info!(target: "init", "Establishing Connection");
    let connack = Server::init(
        &mut s_server,
        server.settings.tls,
        server.settings.compression,
    )
    .await?;

    debug!(target: "init", "{:?}", connack);

    let codec = QuasselCodec::builder().compression(false).new_codec();
    let framed = Framed::new(s_server, codec);
    let (s_sink, s_stream) = framed.split();

    //
    // Accept first listerner
    //

    let listener = TcpListener::bind("0.0.0.0:4243").await.unwrap();
    let (mut client, _) = listener.accept().await.unwrap();

    //
    // Setup Listener
    //

    {
        let (mut c_stream, mut c_sink) = client.split();

        let mut init = [0; 12];
        let n = c_stream.peek(&mut init).await.unwrap();
        c_stream.read(&mut init[..n]).await.unwrap();
        let init = libquassel::message::Init::parse(&init);
        debug!("{:?}", init);

        c_sink.write(&[0x0, 0x0, 0x0, 0x2]).await.unwrap();
    }

    let codec = QuasselCodec::builder().compression(false).new_codec();
    let framed = Framed::new(client, codec);
    let (c_sink, c_stream) = framed.split();

    //
    // Start Processing
    //

    let s_state = ClientState::Handshake;
    let c_state = ClientState::Handshake;

    tokio::join!(
        Server::run(s_stream, c_sink, s_state, "server -> client"),
        Server::run(c_stream, s_sink, c_state, "client -> server")
    );

    Ok(())
}

#[derive(Clone, Debug)]
pub struct ServerSettings {
    pub tls: bool,
    pub compression: bool,
    pub host: String,
    pub port: u32,
    pub username: String,
    pub password: String,
}

pub struct Server {
    server_name: String,
    settings: ServerSettings,
}

impl std::fmt::Debug for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt = f.debug_struct("Server");
        fmt.field("settings", &self.settings);
        fmt.field("name", &self.server_name).finish()
    }
}

#[derive(Clone, Debug)]
pub enum ClientState {
    Handshake,
    Connected,
}

impl Server {
    fn new(name: &str, settings: ServerSettings) -> Self {
        Server {
            server_name: name.to_string(),
            settings,
        }
    }

    // Send the initialization message to the stream
    async fn init(
        stream: &mut (impl AsyncRead + AsyncWrite + Unpin),
        tls: bool,
        compression: bool,
    ) -> Result<ConnAck, Error> {
        use libquassel::deserialize::*;

        // Buffer for our initialization
        let mut init: Vec<u8> = vec![];

        // The handshake message
        let mut handshake: u32 = 0x42b33f00;

        // If TLS is enabled set the TLS bit on the handshake
        if tls {
            info!(target: "init", "Enabled TLS");
            handshake |= 0x01;
        }

        // If COMPRESSION is enabled set the COMPRESSION bit on the handshake
        if compression {
            info!(target: "init", "Enabled Compression");
            handshake |= 0x02;
        }

        // Select Protocol 2: Datastream
        let mut proto: u32 = 0x00000002;

        // Flag proto as the last protocol
        let fin: u32 = 0x80000000;
        proto |= fin;

        // Add handshake and protocol to our buffer
        init.extend(handshake.to_be_bytes().iter());
        init.extend(proto.to_be_bytes().iter());

        // Send Buffer
        stream.write(&init).await?;

        // Read Response
        let mut buf = [0; 4];
        stream.read(&mut buf).await?;

        let (_, connack) = ConnAck::parse(&buf)?;
        Ok(connack)
    }

    pub async fn run(
        mut stream: SplitStream<Framed<TcpStream, QuasselCodec>>,
        mut sink: SplitSink<Framed<TcpStream, QuasselCodec>, Vec<u8>>,
        mut state: ClientState,
        direction: &str,
    ) {
        // Start event loop
        while let Some(msg) = stream.next().await {
            let msg = msg.unwrap();
            sink.send(msg.to_vec()).await.unwrap();
            match state {
                ClientState::Handshake => Server::handle_login_message(&msg, &mut state, direction)
                    .await
                    .unwrap(),
                ClientState::Connected => Server::handle_message(&msg, direction).await.unwrap(),
            }
        }
    }

    #[tracing::instrument]
    async fn handle_login_message(
        buf: &[u8],
        state: &mut ClientState,
        direction: &str,
    ) -> Result<(), Error> {
        use libquassel::HandshakeDeserialize;

        trace!(target: "handshakemessage", "Received bytes: {:x?}", buf);
        match HandshakeMessage::parse(buf) {
            Ok((_size, res)) => {
                info!("{}: {:#?}", direction, res);

                match res {
                    HandshakeMessage::SessionInit(_) => *state = ClientState::Connected,
                    HandshakeMessage::ClientLogin(_) => *state = ClientState::Connected,
                    _ => {}
                }
            }
            Err(e) => error!("failed to parse handshake message {}", e),
        }

        Ok(())
    }

    #[tracing::instrument]
    async fn handle_message(buf: &[u8], direction: &str) -> Result<(), Error> {
        use libquassel::deserialize::*;
        use libquassel::message::Message;

        trace!(target: "message", "Received bytes: {:x?}", buf);

        match Message::parse(buf) {
            Ok((_size, res)) => info!("{}: {:#?}", direction, res),
            Err(e) => error!("failed to parse message {}", e),
        }

        return Ok(());
    }
}
