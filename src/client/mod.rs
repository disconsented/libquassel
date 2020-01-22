//use std::io::BufWriter;
use std::result::Result;
use std::vec::Vec;

use tokio::net::TcpStream;
use tokio::prelude::*;

use tokio_util::codec::{Framed};
use futures_util::stream::StreamExt;
use futures::SinkExt;

use crate::protocol::frame::QuasselCodec;

use failure::Error;

extern crate log;
// use log::{info, warn, debug};

pub struct Client {
    stream: Framed<TcpStream, QuasselCodec>,
    pub tls: bool,
    pub compression: bool,
}

impl Client {
    pub async fn run(&mut self) {
        // TODO while endlessly loops over same stream element
        while let Some(msg) = self.stream.next().await {
            println!("bing");
            let msg = msg.unwrap();
            handle_login_message(self, &msg).await.unwrap();
        };
     }

    pub async fn connect(address: &'static str, port: u64, tls: bool, compression: bool) -> Result<Client, Error> {
        use crate::protocol::primitive::deserialize::Deserialize;
        use crate::protocol::message::ConnAck;
        use crate::protocol::primitive::{StringList};
        use crate::protocol::message::ClientInit;
        use crate::protocol::message::handshake::HandshakeSerialize;

        let mut s = TcpStream::connect(format!("{}:{}", address, port)).await?;

        // Set Features
        let mut init: Vec<u8> = vec![];
        let mut handshake: u32 = 0x42b33f00;
        if tls {
            handshake |= 0x01;
        }
        if compression {
            handshake |= 0x02;
        }
        let mut proto: u32 = 0x00000002;
        let fin: u32 = 0x80000000;
        proto |= fin;
        init.extend(handshake.to_be_bytes().iter());
        init.extend(proto.to_be_bytes().iter());
        s.write(&init).await?;


        let mut buf = [0; 4];
        s.read(&mut buf).await?;
        let (_, val) = ConnAck::parse(&buf).unwrap();
        println!("Received: {:?}", val);

        let codec = QuasselCodec::builder()
            .compression(compression)
            .new_codec();
        let stream = Framed::new(s, codec);

        let mut client = Client {
            stream: stream,
            tls: tls,
            compression: compression,
        };

        let mut features = StringList::new();
        features.push("SynchronizedMarkerLine".to_string());
        features.push("Authenticators".to_string());
        features.push("ExtendedFeatures".to_string());
        let client_init = ClientInit {
            client_version:String::from("Rust 0.0.0"),
            client_date: String::from("1579009211"),
            feature_list: features,
            client_features: 0x00008000,
        };

        client.stream.send(client_init.serialize()?).await?;

        return Ok(client);
    }
}

pub async fn handle_login_message(client: &mut Client, buf: &[u8]) -> Result<(), Error> {
    use crate::protocol::message::ClientLogin;
    use crate::protocol::message::handshake::{HandshakeSerialize, HandshakeDeserialize, VariantMap};
    use crate::protocol::error::ProtocolError;
    use crate::util::get_msg_type;

    let (_, res) = VariantMap::parse(buf)?;
    println!("res {:?}", res);
    let msgtype = get_msg_type(&res["MsgType"])?;
    match msgtype {
        "ClientInitAck" => {
            let login = ClientLogin {user: "audron".to_string(), password: "audron".to_string()};
            client.stream.send(login.serialize()?).await?;
        },
        "ClientInitReject" => { println!("init failed: {:?}", res) },
        "ClientLoginAck" => { println!("login done: {:?}", res) },
        "ClientLoginReject" => { println!("login failed: {:?}", res)},
        _ => bail!(ProtocolError::WrongMsgType)
    }
    return Ok(());
}
