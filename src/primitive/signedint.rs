extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

use std::convert::TryInto;
use std::result::Result;
use std::vec::Vec;

use failure::Error;

use crate::{Deserialize, Serialize};

impl Serialize for i64 {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl Deserialize for i64 {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let mut rdr = Cursor::new(&b[0..8]);
        return Ok((8, rdr.read_i64::<BigEndian>()?));
    }
}

impl Serialize for i32 {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl Deserialize for i32 {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let mut rdr = Cursor::new(&b[0..4]);
        return Ok((4, rdr.read_i32::<BigEndian>()?));
    }
}

impl Serialize for i16 {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl Deserialize for i16 {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let mut rdr = Cursor::new(&b[0..2]);
        return Ok((2, rdr.read_i16::<BigEndian>()?));
    }
}

impl Serialize for i8 {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        Ok(Vec::from(self.to_be_bytes()))
    }
}

impl Deserialize for i8 {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        return Ok((1, b[0].try_into()?));
    }
}
