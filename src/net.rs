use std::io::prelude::*;
//use std::io::BufWriter;
use std::io::{Error};
use std::result::Result;
use std::net::TcpStream;
use std::vec::Vec;

extern crate log;
use log::{info, warn, debug};

use super::types;

pub struct Server {
    TcpStream: TcpStream,
    pub address: &'static str,
    pub port: u32,
    pub tls: bool,
    pub compression: bool,
}

// impl Server {
//     pub fn login(&self, user: &'static str, pass: &'static str) {
//         println!("{:?}", basic::Void);
//     }
// }

pub fn connect(address: &'static str, port: u32, tls: bool, compression: bool) -> Result<Server, Error> {
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

    let mut buf = [0; 4];
    s.read_exact(&mut buf)?;
    println!("Received: {:?}", types::basic::VOID);

    let mut server: Server = Server {
        TcpStream: s,
        address: address,
        port: port,
        tls: tls,
        compression: compression,
    };

    Ok(server)
}
