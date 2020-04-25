extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

use std::result::Result;
use std::vec::Vec;

use failure::Error;

use crate::protocol::error::ProtocolError;
use crate::protocol::primitive::{deserialize, serialize};

impl serialize::Serialize for bool {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok({
            let i = *self as i8;
            Vec::from(i.to_be_bytes())
        })
    }
}
impl deserialize::Deserialize for bool {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        if b[0] == 0 {
            return Ok((1, false));
        } else if b[0] == 1 {
            return Ok((1, true));
        } else {
            bail!(ProtocolError::BoolOutOfRange);
        };
    }
}
impl serialize::Serialize for u64 {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl deserialize::Deserialize for u64 {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let mut rdr = Cursor::new(&b[0..8]);
        return Ok((8, rdr.read_u64::<BigEndian>()?));
    }
}

impl serialize::Serialize for u32 {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl deserialize::Deserialize for u32 {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let mut rdr = Cursor::new(&b[0..4]);
        return Ok((4, rdr.read_u32::<BigEndian>()?));
    }
}

impl serialize::Serialize for u16 {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl deserialize::Deserialize for u16 {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let mut rdr = Cursor::new(&b[0..2]);
        return Ok((2, rdr.read_u16::<BigEndian>()?));
    }
}

impl serialize::Serialize for u8 {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl deserialize::Deserialize for u8 {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        return Ok((1, b[0]));
    }
}
