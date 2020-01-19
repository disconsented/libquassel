use std::io::prelude::*;
//use std::io::BufWriter;
use std::io::{Error};
use std::result::Result;
use std::net::TcpStream;
use std::vec::Vec;
use std::convert::TryInto;
use std::io::Cursor;

use flate2::Compress;
use flate2::Decompress;
use flate2::Compression;
use flate2::FlushCompress;
use flate2::FlushDecompress;
use flate2::read::ZlibDecoder;

extern crate log;
// use log::{info, warn, debug};

use crate::protocol::message;
use crate::protocol::error::ErrorKind;

pub struct Client {
    tcp_stream: TcpStream,
    encoder: Compress,
    decoder: Decompress,
    pub tls: bool,
    pub compression: bool,
}

impl Client {
    pub fn login(&mut self, user: &'static str, pass: &'static str, client: message::ClientInit) {
        use crate::protocol::message::handshake::{HandshakeDeserialize, HandshakeSerialize, HandshakeQRead, VariantMap};
        use crate::protocol::message::handshake::{ClientInitAck, ClientLogin, ClientLoginAck, SessionInit};

        self.write(&client.serialize().unwrap()).unwrap();

        let mut buf: Vec<u8> = [0; 2048].to_vec();
        let len = VariantMap::read(self, &mut buf).unwrap();
        buf.truncate(len);
        let res = ClientInitAck::parse(&buf).unwrap();
        println!("res: {:?}", res);

        let login = ClientLogin {user: user.to_string(), password: pass.to_string()};
        self.write(&login.serialize().unwrap()).unwrap();
        println!("res: {:?}", res);

        let mut buf: Vec<u8> = [0; 2048].to_vec();
        let len = VariantMap::read(self, &mut buf).unwrap();
        buf.truncate(len);
        let _res = ClientLoginAck::parse(&buf).unwrap();

        let mut buf: Vec<u8> = [0; 2048].to_vec();
        let len = VariantMap::read(self, &mut buf).unwrap();
        buf.truncate(len);
        let res = SessionInit::parse(&buf).unwrap();

        println!("res: {:?}", res);
    }
}

impl std::io::Read for Client {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        return self.tcp_stream.read(buf);
    }
}

impl std::io::Write for Client {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let mut cbuf = Vec::with_capacity(buf.len());
        self.encoder.compress_vec(buf, &mut cbuf, FlushCompress::Finish)?;
        return self.tcp_stream.write(&buf);
    }

    fn flush(&mut self) -> Result<(), Error> {
       return self.tcp_stream.flush();
    }
}

pub fn connect(address: &'static str, port: u32, tls: bool, compression: bool) -> Result<Client, Error> {
    use crate::protocol::primitive::deserialize::Deserialize;

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
        fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
            let (flen, flags) = u8::parse(b)?;
            let (elen, extra) = i16::parse(&b[flen..])?;
            let (vlen, version) = i8::parse(&b[(flen+elen)..])?;

            return Ok((flen+elen+vlen, Self {flags, extra, version}));
        }
    }

    let mut buf = [0; 4];
    s.read(&mut buf)?;
    let (_, val) = ConnAck::parse(&buf).unwrap();
    println!("Received: {:?}", val);

//    let sock = ZlibDecoder::new_with_buf(s, [0; 1].to_vec());
    let server: Client = Client {
        tcp_stream: s,
        encoder: Compress::new(Compression::best(), true),
        decoder: Decompress::new(true),
        tls: tls,
        compression: compression,
    };

    Ok(server)
}
