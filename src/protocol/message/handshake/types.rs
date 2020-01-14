use std::io::Read;
use std::vec::Vec;
use std::net::TcpStream;
use std::convert::TryInto;
use std::collections::HashMap;

use crate::util;
use crate::protocol::primitive::{String, Variant};
use crate::protocol::primitive::serialize::Serialize;
use crate::protocol::primitive::deserialize::Deserialize;
use crate::protocol::primitive::qread::QRead;

pub trait HandshakeSerialize {
    fn serialize(&self) -> Vec<u8>;
}

pub trait HandshakeDeserialize {
    fn parse(b: &[u8]) -> (usize, Self);
}

pub trait HandshakeQRead {
    fn read(stream: &mut std::net::TcpStream, buf: &mut [u8]) -> usize;
}


pub type VariantMap = HashMap<String, Variant>;

impl HandshakeSerialize for VariantMap {
    fn serialize<'a>(&'a self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();

        for (k, v) in self {
            let key = Variant::String(k.clone());
            res.extend(key.serialize());
            res.extend(v.serialize());
        }

        util::insert_bytes(0, &mut res, &mut [0, 0, 0, 10]);

        let len: i32 = res.len().try_into().unwrap();
        util::insert_bytes(0, &mut res, &mut ((len + 4).to_be_bytes()));
        println!("len: {:?}", len + 4);

        return res;
    }
}

impl HandshakeDeserialize for VariantMap {
    fn parse(b: &[u8]) -> (usize, Self) {
        let (_, len) = i32::parse(&b[0..4]);

        let mut pos: usize = 8;
        let mut map = VariantMap::new();
        let ulen: usize = len as usize;
        loop {
            if (pos) >= ulen { break; }
            let (nlen, name) = Variant::parse(&b[(pos)..]);
            pos += nlen;

            let (vlen, value) = Variant::parse(&b[(pos)..]);
            pos += vlen;

            if let Variant::StringUTF8(x) = name {
                map.insert(x, value);
            }
        }

        return (pos, map);
    }
}

impl HandshakeQRead for VariantMap {
    fn read(mut s: &mut TcpStream, b: &mut [u8]) -> usize {
        s.read(&mut b[0..4]).unwrap();
        let (_, len) = i32::parse(&b[0..4]);
        println!("len: {:?}", len);

        // Read the 00 00 00 0a VariantType bytes and discard
        let mut tbuf = [0; 4];
        s.read(&mut tbuf).unwrap();

        let mut pos = 4;
        let len: usize = len as usize;
        loop {
            if pos >= (len - 4) { break; }
            pos += Variant::read(&mut s, &mut b[pos..]);
            pos += Variant::read(&mut s, &mut b[(pos+4..)]);
        }

        return pos;
    }
}
