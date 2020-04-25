use std::collections::HashMap;
use std::convert::TryInto;
use std::result::Result;
use std::vec::Vec;

use failure::Error;

use crate::protocol::error::ProtocolError;
use crate::protocol::primitive::deserialize::Deserialize;
use crate::protocol::primitive::serialize::Serialize;
use crate::protocol::primitive::{String, Variant};
use crate::util;

pub trait HandshakeSerialize {
    fn serialize(&self) -> Result<Vec<u8>, Error>;
}

pub trait HandshakeDeserialize {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error>
    where
        Self: std::marker::Sized;
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
                _ => bail!(ProtocolError::WrongVariant),
            };
        }

        return Ok((pos, map));
    }
}
