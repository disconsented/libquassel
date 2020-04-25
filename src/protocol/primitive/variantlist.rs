use std::convert::TryInto;
use std::vec::Vec;

use failure::Error;

use log::trace;

use crate::protocol::primitive::{deserialize::Deserialize, serialize::Serialize};

extern crate bytes;

use crate::protocol::primitive::Variant;

pub type VariantList = Vec<Variant>;

impl Serialize for VariantList {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let len: i32 = self.len().try_into()?;
        let mut res: Vec<u8> = Vec::new();

        res.extend(len.to_be_bytes().iter());
        for v in self {
            res.extend(v.serialize()?.iter());
        }

        return Ok(res);
    }
}

impl Deserialize for VariantList {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (_, len) = i32::parse(&b[0..4])?;
        trace!(target: "protocol::primitive::VariantList", "Parsing VariantList with {:?} elements", len);

        let mut res: VariantList = VariantList::new();
        let mut pos: usize = 4;
        for i in 0..len {
            trace!(target: "protocol::primitive::VariantList", "Parsing VariantList element: {:?}", i);
            let (vlen, val) = Variant::parse(&b[pos..])?;
            res.push(val);
            pos += vlen;
        }

        return Ok((pos, res));
    }
}
