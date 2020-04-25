extern crate byteorder;

use std::result::Result;
use std::vec::Vec;

use failure::Error;

use log::trace;

use crate::protocol::primitive::{deserialize, serialize};
use crate::util;

pub type String = std::string::String;
impl serialize::Serialize for String {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut res: Vec<u8> = Vec::new();

        let utf16: Vec<u16> = self.encode_utf16().collect();
        for i in utf16 {
            res.extend(i.to_be_bytes().iter());
        }

        util::prepend_byte_len(&mut res);
        return Ok(res);
    }
}

impl serialize::SerializeUTF8 for String {
    fn serialize_utf8(&self) -> Result<Vec<u8>, Error> {
        let mut res: Vec<u8> = Vec::new();
        res.extend(self.clone().into_bytes());
        res.extend(vec![0x00]);
        util::prepend_byte_len(&mut res);
        return Ok(res);
    }
}

impl deserialize::Deserialize for String {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        // Parse Length
        let (_, len) = i32::parse(&b[0..4])?;
        trace!(target: "protocol::primitive::String", "Parsing with length: {:?}, from bytes: {:x?}", len, &b[0..4]);

        if len == -1 {
            return Ok((4, "".to_string()));
        }

        // length as usize
        let ulen = len as usize;
        let mut pos: usize = 4;
        let mut chars: Vec<u16> = Vec::new();
        loop {
            // if position is behind the length plus our 4 bytes of the length we already parsed
            if pos >= (ulen + 4) {
                break;
            }
            let (slen, uchar) = u16::parse(&b[pos..(pos + 2)])?;
            chars.push(uchar);
            pos += slen;
        }

        let res: String = String::from_utf16(&chars).unwrap();
        return Ok((pos, res));
    }
}

impl deserialize::DeserializeUTF8 for String {
    fn parse_utf8(b: &[u8]) -> Result<(usize, Self), Error> {
        use crate::protocol::primitive::deserialize::Deserialize;
        let (_, len) = i32::parse(&b[0..4])?;

        trace!(target: "protocol::primitive::String", "Parsing with length: {:?}, from bytes: {:x?}", len, &b[0..4]);

        if len <= 0 {
            return Ok((4, "".to_string()));
        }

        let ulen = len as usize;

        let mut res: String = String::from_utf8(b[4..(ulen + 4)].to_vec())?;

        // If the last byte is zero remove it
        // Receiving a string as bytearray will sometimes have
        // the string null terminated
        if res.chars().last().unwrap() == '\u{0}' {
            let _ = res.pop();
        }

        return Ok((ulen + 4, res));
    }
}
