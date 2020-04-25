extern crate byteorder;

use std::convert::TryInto;
use std::result::Result;
use std::vec::Vec;

use failure::Error;

use log::trace;

use crate::protocol::primitive::{deserialize, serialize};

pub type StringList = Vec<String>;
impl serialize::Serialize for StringList {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let len: i32 = self.len().try_into()?;
        let mut res: Vec<u8> = Vec::new();

        res.extend(len.to_be_bytes().iter());
        for x in self {
            res.extend(x.serialize()?);
        }

        return Ok(res);
    }
}

impl deserialize::Deserialize for StringList {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (_, len) = i32::parse(&b[0..4])?;
        trace!(target: "protocol::primitive::StringList", "Parsing with length: {:?}, from bytes: {:x?}", len, &b[0..4]);
        let mut res: StringList = StringList::new();

        let mut pos = 4;
        if len > 0 {
            for _ in 0..len {
                let (lpos, val) = String::parse(&b[pos..])?;
                pos += lpos;
                res.push(val);
            }
        }

        return Ok((pos, res));
    }
}
