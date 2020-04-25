extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

use std::convert::TryInto;
use std::result::Result;
use std::vec::Vec;

use failure::Error;

use crate::protocol::primitive::{deserialize, serialize};

impl serialize::Serialize for i64 {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl deserialize::Deserialize for i64 {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let mut rdr = Cursor::new(&b[0..8]);
        return Ok((8, rdr.read_i64::<BigEndian>()?));
    }
}

impl serialize::Serialize for i32 {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl deserialize::Deserialize for i32 {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let mut rdr = Cursor::new(&b[0..4]);
        return Ok((4, rdr.read_i32::<BigEndian>()?));
    }
}

impl serialize::Serialize for i16 {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl deserialize::Deserialize for i16 {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let mut rdr = Cursor::new(&b[0..2]);
        return Ok((2, rdr.read_i16::<BigEndian>()?));
    }
}

impl serialize::Serialize for i8 {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl deserialize::Deserialize for i8 {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        return Ok((1, b[0].try_into()?));
    }
}
