use std::io::Read;
use std::vec::Vec;
use std::result::Result;
use std::convert::TryInto;
use std::collections::HashMap;

use failure::Error;

use crate::util;
use crate::protocol::primitive::{String, Variant};
use crate::protocol::primitive::serialize::Serialize;
use crate::protocol::primitive::deserialize::Deserialize;
use crate::protocol::primitive::qread::QRead;
use crate::protocol::error::ProtocolError;

pub trait HandshakeSerialize {
    fn serialize(&self) -> Result<Vec<u8>, Error>;
}

pub trait HandshakeDeserialize {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> where Self: std::marker::Sized ;
}

pub trait HandshakeQRead {
    fn read<T: Read>(stream: &mut T, buf: &mut [u8]) -> Result<usize, Error>;
}

pub type VariantMap = HashMap<String, Variant>;

impl HandshakeSerialize for VariantMap {
    fn serialize<'a>(&'a self) -> Result<Vec<u8>, Error> {
        let mut res: Vec<u8> = Vec::new();

        for (k, v) in self {
            let key = Variant::String(k.clone());
            res.extend(key.serialize()?);
            res.extend(v.serialize()?);
        }

        let len: i32 = (self.len() * 2).try_into().unwrap();
        util::insert_bytes(0, &mut res, &mut (len).to_be_bytes());

        return Ok(res);
    }
}

impl HandshakeDeserialize for VariantMap {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (_, len) = i32::parse(&b[0..4])?;

        let mut pos: usize = 4;
        let mut map = VariantMap::new();

        for _ in 0..(len / 2) {
            let (nlen, name) = Variant::parse(&b[pos..])?;
            pos += nlen;

            let (vlen, value) = Variant::parse(&b[pos..])?;
            pos += vlen;

            match name {
                Variant::String(x) => map.insert(x, value),
                Variant::StringUTF8(x) => map.insert(x, value),
                _ => bail!(ProtocolError::WrongVariant)
            };
        }

        return Ok((pos, map));
    }
}

impl HandshakeQRead for VariantMap {
    fn read<T: Read>(s: &mut T, b: &mut [u8]) -> Result<usize, Error> {
        s.read(&mut b[0..4])?;
        let (_, len) = i32::parse(&b[0..4])?;

        let mut pos = 4;
        for _ in 0..(len / 2) {
            pos += Variant::read(s, &mut b[pos..])?;
            pos += Variant::read(s, &mut b[pos..])?;
        }

//        let mut pos = 8;
//        let len: usize = len as usize;
//        loop {
//            if pos >= len { break; }
//            pos += Variant::read(s, &mut b[pos..])?;
//            pos += Variant::read(s, &mut b[pos..])?;
//        }

        return Ok(pos);
    }
}