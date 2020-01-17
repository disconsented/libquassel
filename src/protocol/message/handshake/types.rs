use std::io::Read;
use std::vec::Vec;
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
    fn read<T: Read>(stream: &mut T, buf: &mut [u8]) -> usize;
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
        util::insert_bytes(0, &mut res, &mut ((len).to_be_bytes()));

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
            let (nlen, name) = Variant::parse(&b[pos..]);
            pos += nlen;

            let (vlen, value) = Variant::parse(&b[pos..]);
            pos += vlen;

            match name {
                Variant::String(x) => map.insert(x, value),
                Variant::StringUTF8(x) => map.insert(x, value),
                _ => panic!()
            };
        }

        return (pos, map);
    }
}

impl HandshakeQRead for VariantMap {
    fn read<T: Read>(s: &mut T, b: &mut [u8]) -> usize {
        s.read(&mut b[0..4]).unwrap();
        let (_, len) = i32::parse(&b[0..4]);

        // Read the 00 00 00 0a VariantType bytes and discard
        s.read(&mut b[4..8]).unwrap();

        let mut pos = 8;
        let len: usize = len as usize;
        loop {
            if pos >= (len - 4) { break; }
            pos += Variant::read(s, &mut b[pos..]);
            pos += Variant::read(s, &mut b[pos..]);
        }

        return pos;
    }
}
