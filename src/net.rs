use std::io::prelude::*;
//use std::io::BufWriter;
use std::io::{Error};
use std::result::Result;
use std::net::TcpStream;
use std::vec::Vec;

extern crate log;
// use log::{info, warn, debug};


use crate::protocol::message;

pub struct Client {
    tcp_stream: TcpStream,
    pub address: &'static str,
    pub port: u32,
    pub tls: bool,
    pub compression: bool,
}

impl Client {
    pub fn login(&mut self, user: &'static str, pass: &'static str, client: message::ClientInit) {
        use crate::protocol::message::handshake::{HandshakeDeserialize, HandshakeSerialize, HandshakeQRead, VariantMap};
        use crate::protocol::message::handshake::{ClientInit, ClientInitAck};
        use std::convert::TryInto;

        let sclientinit = &client.serialize();
        let len: u32 = sclientinit.len().try_into().unwrap();
//        self.tcp_stream.write(&len.to_be_bytes()).unwrap();
        self.tcp_stream.write(sclientinit).unwrap();

        let mut buf: Vec<u8> = [0; 2048].to_vec();
        VariantMap::read(&mut self.tcp_stream, &mut buf);

        // println!("{:?}", buf);
        let res = ClientInitAck::parse(&buf);
        println!("{:?}", res)
    }
}

pub fn connect(address: &'static str, port: u32, tls: bool, compression: bool) -> Result<Client, Error> {
    use crate::protocol::primitive::serialize::Serialize;
    use crate::protocol::primitive::deserialize::Deserialize;
    use crate::protocol::primitive::qread::QRead;

    //let mut s = BufWriter::new(TcpStream::connect(format!("{}:{}", address, port)).unwrap());
    let mut s = TcpStream::connect(format!("{}:{}", address, port)).unwrap();

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
    s.write(&init)?;

    #[derive(Debug)]
    struct ConnAck {
        flags: u8,
        extra: i16,
        version: i8
    }

    impl Deserialize for ConnAck {
        fn parse(b: &[u8]) -> (usize, Self) {
            let (flen, flags) = u8::parse(b);
            let (elen, extra) = i16::parse(&b[flen..]);
            let (vlen, version) = i8::parse(&b[(flen+elen)..]);

            return (flen+elen+vlen, Self {flags, extra, version});
        }
    }

    let mut buf = [0; 4];
    s.read_exact(&mut buf)?;
    let (_, val) = ConnAck::parse(&buf);
    println!("Received: {:?}", val);

    let server: Client = Client {
        tcp_stream: s,
        address: address,
        port: port,
        tls: tls,
        compression: compression,
    };

    Ok(server)
}
