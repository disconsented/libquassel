use std::collections::HashMap;
use std::{convert::TryInto, vec::Vec};

use failure::Error;

use log::trace;

use crate::protocol::error::ProtocolError;
use crate::protocol::primitive::deserialize::Deserialize;
use crate::protocol::primitive::serialize::Serialize;
use crate::protocol::primitive::String;

use crate::protocol::primitive::Variant;
use crate::util;

extern crate bytes;

pub type VariantMap = HashMap<String, Variant>;

impl Serialize for VariantMap {
    fn serialize<'a>(&'a self) -> Result<Vec<u8>, Error> {
        let mut res: Vec<u8> = Vec::new();

        for (k, v) in self {
            res.extend(k.serialize()?);
            res.extend(v.serialize()?);
        }

        let len: i32 = self.len().try_into()?;
        util::insert_bytes(0, &mut res, &mut len.to_be_bytes());

        return Ok(res);
    }
}

impl Deserialize for VariantMap {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (_, len) = i32::parse(&b[0..4])?;
        trace!(target: "protocol::primitive::VariantMap", "Parsing VariantMap with {:?} elements", len);

        let mut pos: usize = 4;
        let mut map = VariantMap::new();
        for _ in 0..len {
            trace!(target: "protocol::primitive::VariantMap", "Parsing entry name");
            // let (nlen, name) = Variant::parse(&b[pos..])?;
            let (nlen, name) = String::parse(&b[pos..])?;
            pos += nlen;

            trace!(target: "protocol::primitive::VariantMap", "Parsing entry: {:?} with len {:?}", name, &b[(pos)..(pos + 4)]);
            let (vlen, value) = Variant::parse(&b[(pos)..])?;
            pos += vlen;

            // match name {
            //     Variant::String(x) => map.insert(x, value),
            //     Variant::StringUTF8(x) => map.insert(x, value),
            //     _ => bail!(ProtocolError::WrongVariant),
            // };
            map.insert(name, value);
        }

        return Ok((pos, map));
    }
}
