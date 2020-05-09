//use std::io::BufWriter;
use std::result::Result;
use std::vec::Vec;

use tokio::io::{AsyncRead, AsyncWrite};
use core::marker::Unpin;
use tokio::net::TcpStream;
use tokio::prelude::*;

use native_tls::TlsConnector;

use tokio_tls;
use tokio_tls::TlsStream;
use tokio_util::codec::Framed;
use futures_util::stream::StreamExt;
use futures::SinkExt;
use std::future::Future;

use crate::frame::QuasselCodec;

use failure::Error;

use log::{trace, debug, info, error};

use crate::message::*;
use crate::primitive::*;

extern crate log;

pub struct Client<T, F>
where
    F: Future,
    T: 'static + AsyncRead + AsyncWrite + Unpin,
{
    pub stream: Framed<T, QuasselCodec>,
    pub tls: bool,
    pub compression: bool,
    pub state: ClientState,
    pub user: User,
    pub funcs: Funcs<T, F>,
}

pub struct User {
    pub name: String,
    pub password: String,
}

pub enum ClientState {
    Handshake,
    Connected,
}

impl <T: AsyncRead + AsyncWrite + Unpin, F: Future> Client<T, F> {
    pub async fn run(&mut self) {
        use crate::primitive::StringList;
        use crate::message::ClientInit;
        use crate::HandshakeSerialize;

        info!(target: "init", "Setting Features");

        let mut features = StringList::new();
        features.push("SynchronizedMarkerLine".to_string());
        features.push("Authenticators".to_string());
        features.push("ExtendedFeatures".to_string());
        features.push("BufferActivitySync".to_string());
        let client_init = ClientInit {
            client_version:String::from("Rust 0.0.0"),
            client_date: String::from("1579009211"),
            feature_list: features,
            client_features: 0x00008000,
        };

        self.stream.send(client_init.serialize().unwrap()).await.unwrap();

        // Start event loop
        while let Some(msg) = self.stream.next().await {
            let msg = msg.unwrap();
            match self.state {
                ClientState::Handshake => handle_login_message(self, &msg).await.unwrap(),
                ClientState::Connected => handle_message(self, &msg).await.unwrap(),
            }
        };
    }

    pub async fn connect(
        address: &'static str,
        port: u64,
        compression: bool,
        user: User,
        funcs: Funcs<TcpStream, impl Future>
    ) -> Result<Client<TcpStream, impl Future>, Error> {
        let mut stream = TcpStream::connect(format!("{}:{}", address, port)).await?;

        info!(target: "init", "Establishing Connection");
        let connack = init(&mut stream, false, compression).await?;

        debug!(target: "init", "{:?}", connack);

        let codec = QuasselCodec::builder()
            .compression(compression)
            .new_codec();

        let framed_stream = Framed::new(stream, codec);

        info!(target: "init", "Established Connection");

        return Ok(Client {
            stream: framed_stream,
            tls: false,
            compression,
            state: ClientState::Handshake,
            user,
            funcs,
        });
    }

    pub async fn connect_tls(
        address: &'static str,
        port: u64,
        compression: bool,
        user: User,
        funcs: Funcs<TlsStream<TcpStream>, impl Future>
    ) -> Result<Client<TlsStream<TcpStream>, impl Future>, Error> {
        let mut stream: TcpStream = TcpStream::connect(format!("{}:{}", address, port)).await?;

        info!(target: "init", "Establishing Connection");
        let connack = init(&mut stream, true, compression).await?;

        debug!(target: "init", "{:?}", connack);

        let codec = QuasselCodec::builder()
            .compression(compression)
            .new_codec();

        let tls_connector = tokio_tls::TlsConnector::from(TlsConnector::builder().build().unwrap());

        let tls_stream = tls_connector.connect(address, stream).await?;

        let framed_stream = Framed::new(tls_stream, codec);

        info!(target: "init", "Established Connection");

        return Ok(Client {
            stream: framed_stream,
            tls: true,
            compression,
            state: ClientState::Handshake,
            user,
            funcs,
        });
    }

}

pub async fn handle_login_message<T: AsyncRead + AsyncWrite + Unpin, F: Future>(client: &mut Client<T, F>, buf: &[u8]) -> Result<(), Error> {
    use crate::{HandshakeSerialize, HandshakeDeserialize};
    use crate::message::ClientLogin;
    use crate::primitive::{VariantMap, Variant};

    trace!(target: "message", "Received bytes: {:x?}", buf);
    let (_, res) = VariantMap::parse(buf)?;
    debug!(target: "init", "Received Messsage: {:#?}", res);

    let msgtype = match_variant!(&res["MsgType"], Variant::String);
    match msgtype.as_str() {
        "ClientInitAck" => {
            info!(target: "init", "Initialization successfull");
            info!(target: "login", "Starting Login");
            let login = ClientLogin {user: client.user.name.clone(), password: client.user.password.clone()};
            client.stream.send(login.serialize()?).await?;
        },
        "ClientInitReject" => {
            error!(target: "init", "Initialization failed: {:?}", res);
        },
        "ClientLoginAck" => {
            info!(target: "login", "Login successfull");
        },
        "SessionInit" => {
            info!(target: "login", "Session Initialization finished. Switching to Connected state");
            client.state = ClientState::Connected;
        }
        "ClientLoginReject" => {
            error!(target: "login", "Login failed: {:?}", res);
        },
        _ => {
            error!(target: "client", "Error: WrongMsgType: {:#?}", res);
        }
    }

    return Ok(());
}

pub async fn handle_message<T: AsyncRead + AsyncWrite + Unpin, F: Future>(client: &mut Client<T, F>, buf: &[u8]) -> Result<(), Error> {
    use crate::message::Message;
    use crate::primitive::VariantList;
    use crate::Deserialize;
    use crate::Serialize;

    trace!(target: "message", "Received bytes: {:x?}", buf);
    let (_, res) = Message::parse(buf)?;
    debug!(target: "init", "Received Messsage: {:#?}", res);

    match res {
        Message::SyncMessage(_) => {}
        Message::RpcCall(_) => {}
        Message::InitRequest(_) => {}
        Message::InitData(_) => {}
        Message::HeartBeat(_) => {}
        Message::HeartBeatReply(_) => {}
    }

    return Ok(());
}

pub struct Funcs<T, F>
where
    T: 'static + AsyncRead + AsyncWrite + Unpin,
    F: std::future::Future,
{
    pub init: InitFuncs<T, F>,
    pub message: MessageFuncs<T, F>,
}

pub struct InitFuncs<T, F>
where
    T: 'static + AsyncRead + AsyncWrite + Unpin,
    F: std::future::Future,
{
   pub client_init_ack: fn(&mut Client<T, F>, VariantMap) -> F,
   pub client_init_reject: fn(Client<T, F>, VariantMap) -> F,
   pub client_login_ack: fn(Client<T, F>, VariantMap) -> F,
   pub client_login_reject: fn(Client<T, F>, VariantMap) -> F,
   pub session_init: fn(&mut Client<T, F>, VariantMap) -> F,
}

pub struct MessageFuncs<T, F>
where
    T: 'static + AsyncRead + AsyncWrite + Unpin,
    F: std::future::Future,
{
    pub sync_message: fn(Client<T, F>, SyncMessage) -> F,
    pub rpc_call: fn(Client<T, F>, RpcCall) -> F,
    pub init_request: fn(Client<T, F>, InitRequest) -> F,
    pub init_data: fn(Client<T, F>, InitData) -> F,
    pub heart_beat: fn(Client<T, F>, HeartBeat) -> F,
    pub heart_beat_reply: fn(Client<T, F>, HeartBeatReply) -> F,
}

// Send the initialization message to the stream
pub async fn init(stream: &mut TcpStream, tls: bool, compression: bool) -> Result<ConnAck, Error> {
    use crate::Deserialize;

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
