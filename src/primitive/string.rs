extern crate byteorder;

use std::result::Result;
use std::vec::Vec;

use failure::Error;

use log::trace;

use crate::{deserialize::*, error::ProtocolError, serialize::*, util};

impl Deserialize for char {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (slen, qchar): (usize, u16) = u16::parse(&b[0..2])?;
        let qchar = char::from_u32(qchar as u32).ok_or(ProtocolError::CharError)?;

        return Ok((slen, qchar));
    }
}

impl Serialize for char {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut b = [0, 0];
        self.encode_utf16(&mut b);

        return Ok(b[0].to_be_bytes().to_vec());
    }
}

/// We Shadow the String type here as we can only use impl on types in our own scope.
///
/// Strings are serialized as an i32 for the length in bytes, then the chars represented in UTF-16 in bytes.
///
/// Strings can only be serialized as UTF-8 null-terminated ByteArrays with (de)serialize_utf8().
impl Serialize for String {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut res: Vec<u8> = Vec::new();

        self.encode_utf16()
            .for_each(|i| res.extend(i.to_be_bytes().iter()));

        util::prepend_byte_len(&mut res);
        return Ok(res);
    }
}

impl SerializeUTF8 for String {
    fn serialize_utf8(&self) -> Result<Vec<u8>, Error> {
        let mut res: Vec<u8> = Vec::new();
        res.extend(self.clone().into_bytes());
        util::prepend_byte_len(&mut res);
        return Ok(res);
    }
}

impl Deserialize for String {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        // Parse Length
        let (_, len) = i32::parse(&b[0..4])?;
        trace!(target: "primitive::String", "Parsing with length: {:?}, from bytes: {:x?}", len, &b[0..4]);

        if len == -1 {
            return Ok((4, "".to_string()));
        }

        // length as usize
        let ulen = len as usize;
        trace!("parsed bytes: {:x?}", &b[0..ulen]);
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
        trace!("parsed string: {}", res);
        return Ok((pos, res));
    }
}

impl DeserializeUTF8 for String {
    fn parse_utf8(b: &[u8]) -> Result<(usize, Self), Error> {
        let (_, len) = i32::parse(&b[0..4])?;

        trace!(target: "primitive::String", "Parsing with length: {:?}, from bytes: {:x?}", len, &b[0..4]);

        if len <= 0 {
            return Ok((4, "".to_string()));
        }

        let ulen = len as usize;

        let mut res: String = String::from_utf8(b[4..(ulen + 4)].to_vec())?;
        trace!("parsed string: {}", res);

        // If the last byte is zero remove it
        // Receiving a string as bytearray will sometimes have
        // the string null terminated
        if res.chars().last().unwrap() == '\u{0}' {
            let _ = res.pop();
        }

        trace!("parsed string after trunc: {}", res);
        trace!("parsed bytes: {:x?}", &b[0..ulen]);

        return Ok((ulen + 4, res));
    }
}

#[test]
pub fn string_serialize() {
    let test_string: String = String::from("Configured");

    assert_eq!(
        test_string.serialize().unwrap(),
        [
            0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0,
            100
        ]
    );
}

#[test]
pub fn string_serialize_utf8() {
    let test_string: String = String::from("Configured");

    assert_eq!(
        test_string.serialize_utf8().unwrap(),
        [0, 0, 0, 10, 67, 111, 110, 102, 105, 103, 117, 114, 101, 100]
    );
}

#[test]
pub fn string_deserialize() {
    let test_bytes: &[u8] = &[
        0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0, 100,
        0, 0, 0, 1,
    ];
    let (len, res) = String::parse(test_bytes).unwrap();
    assert_eq!(res, "Configured");
    assert_eq!(len, 24);
}

#[test]
pub fn string_deserialize_utf8() {
    let test_bytes: &[u8] = &[
        0, 0, 0, 10, 67, 111, 110, 102, 105, 103, 117, 114, 101, 100, 0, 0, 0, 1,
    ];
    let (len, res) = String::parse_utf8(test_bytes).unwrap();
    assert_eq!(len, 14);
    assert_eq!(res, "Configured");
}

#[test]
pub fn string_deserialize_utf8_null_terminated() {
    let test_bytes: &[u8] = &[
        0, 0, 0, 11, 67, 111, 110, 102, 105, 103, 117, 114, 101, 100, 0, 0, 0, 0, 1,
    ];
    let (len, res) = String::parse_utf8(test_bytes).unwrap();
    assert_eq!(len, 15);
    assert_eq!(res, "Configured");
}
