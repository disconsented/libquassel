//use std::io::BufWriter;
use std::result::Result;
use std::vec::Vec;
use std::convert::TryInto;
use std::io::Cursor;

use flate2::Compress;
use flate2::Decompress;
use flate2::Compression;
use flate2::FlushCompress;
use flate2::FlushDecompress;
use flate2::read::ZlibDecoder;

use tokio::net::TcpStream;
use tokio::prelude::*;

use failure::Error;

extern crate log;
// use log::{info, warn, debug};

use crate::protocol::message;

pub enum State {
    Handshake,
    Connected
}

pub struct Client {
    tcp_stream: TcpStream,
    encoder: Compress,
    decoder: Decompress,
    state: State,
    pub tls: bool,
    pub compression: bool,
}

impl Client {
    pub async fn handler(mut self) -> Result<(), Error> {
//        let (recv, send) = self.tcp_stream.split();
        loop {
            let mut buf: Vec<u8> = vec![0; 2048];
            match self.tcp_stream.read(&mut buf).await {
                Ok(n) => {
                    buf.truncate(n);
                    let mut cbuf: Vec<u8> = vec![0; n * 2];

                    println!("buf: {:?}", &buf[0..]);
                    let before_in = self.decoder.total_in();
                    let before_out = self.decoder.total_out();
                    self.decoder.decompress(&buf, &mut cbuf, FlushDecompress::None)?;
                    let after_in = self.decoder.total_in();
                    let after_out = self.decoder.total_out();

                    cbuf.truncate(after_out.try_into()?);

                    println!("in: {:?} / {:?}\nout: {:?} / {:?}", before_in, after_in, before_out, after_out);

                    println!("buf: {:?}", cbuf);

                    match self.state {
                        State::Handshake => handle_login_message(&mut self, &cbuf),
                        State::Connected => handle_login_message(&mut self, &cbuf)
                    }.await?;
                }
                Err(e) => { panic!(e) }
            }
        }
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

        let mut client = Client {
            tcp_stream: s,
            state: State::Handshake,
            encoder: Compress::new(Compression::best(), true),
            decoder: Decompress::new(true),
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
        write_to_stream(&mut client, &client_init.serialize()?).await?;

        return Ok(client);
    }

//    pub fn login(&mut self, user: &'static str, pass: &'static str, client: message::ClientInit) {
//        use crate::protocol::message::handshake::{HandshakeDeserialize, HandshakeSerialize, HandshakeQRead, VariantMap};
//        use crate::protocol::message::handshake::{ClientInitAck, ClientLogin, ClientLoginAck, SessionInit};
//
//        self.write(&client.serialize().unwrap()).unwrap();
//
//        let mut buf: Vec<u8> = [0; 2048].to_vec();
//        let len = VariantMap::read(self, &mut buf).unwrap();
//        buf.truncate(len);
//        let res = ClientInitAck::parse(&buf).unwrap();
//        println!("res: {:?}", res);
//
//        let login = ClientLogin {user: user.to_string(), password: pass.to_string()};
//        self.write(&login.serialize().unwrap()).unwrap();
//        println!("res: {:?}", res);
//
//        let mut buf: Vec<u8> = [0; 2048].to_vec();
//        let len = VariantMap::read(self, &mut buf).unwrap();
//        buf.truncate(len);
//        let _res = ClientLoginAck::parse(&buf).unwrap();
//
//        let mut buf: Vec<u8> = [0; 2048].to_vec();
//        let len = VariantMap::read(self, &mut buf).unwrap();
//        buf.truncate(len);
//        let res = SessionInit::parse(&buf).unwrap();
//
//        println!("res: {:?}", res);
//    }
}

// impl std::io::Read for Client {
//     fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
//         let mut cbuf = [0; 2048].to_vec();
//         let read_bytes = self.tcp_stream.read(&mut cbuf)?;
//         println!("read bytes: {:?}", read_bytes);
//         cbuf.truncate(read_bytes);
//         println!("cbuf: {:?}", &cbuf[0..]);
//         let before_in = self.decoder.total_in();
//         let before_out = self.decoder.total_out();
//         self.decoder.decompress(&cbuf, buf, FlushDecompress::None)?;
//         let after_in = self.decoder.total_in();
//         let after_out = self.decoder.total_out();
//
//         println!("in: {:?} / {:?}\nout: {:?} / {:?}", before_in, after_in, before_out, after_out);
//
//         println!("buf: {:?}", buf);
//         return Ok(((after_in - after_out)).try_into().unwrap());
// //
// //        let res = self.tcp_stream.read(buf);
// //        println!("buf: {:?}, total in: {:?}, total out: {:?}", buf, self.tcp_stream.total_in(), self.tcp_stream.total_out());
// //        return res;
//     }
// }
//
// impl std::io::Write for Client {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
//         let mut cbuf = Vec::with_capacity(buf.len());
//         self.encoder.compress_vec(buf, &mut cbuf, FlushCompress::Finish)?;
//         return self.tcp_stream.write(&cbuf);
//     }
//
//     fn flush(&mut self) -> Result<(), Error> {
//        return self.tcp_stream.flush();
//     }
// }

pub async fn write_to_stream(client: &mut Client, buf: &[u8]) -> Result<usize, Error> {
    let mut cbuf = Vec::with_capacity(buf.len());
    client.encoder.compress_vec(buf, &mut cbuf, FlushCompress::Finish)?;
    return Ok(client.tcp_stream.write(&cbuf).await?);
}

pub async fn handle_login_message(client: &mut Client, buf: &[u8]) -> Result<(), Error> {
    use crate::protocol::primitive::{Variant, VariantMap, StringList};
    use crate::protocol::message::ClientLogin;
    use crate::protocol::message::handshake::HandshakeSerialize;
    use crate::protocol::primitive::deserialize::Deserialize;
    use crate::protocol::error::ProtocolError;
    use crate::util::get_msg_type;

    let (_, res) = VariantMap::parse(buf)?;
    let msgtype = get_msg_type(&res["MsgType"])?;
    match msgtype {
        "ClientInitAck" => {
            let login = ClientLogin {user: "audron".to_string(), password: "audron".to_string()};
            write_to_stream(client, &login.serialize()?).await?;
        },
        "ClientInitReject" => { println!("init failed: {:?}", res) },
        "ClientLoginAck" => { println!("login done: {:?}", res) },
        "ClientLoginReject" => { println!("login failed: {:?}", res)},
        _ => bail!(ProtocolError::WrongMsgType)
    }
    return Ok(());
}
