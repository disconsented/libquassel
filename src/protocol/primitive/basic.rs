#[allow(dead_code)]
pub const VOID: u32         = 0x00000000;
pub const BOOL: u32         = 0x00000001;
pub const QCHAR: u32        = 0x00000007;

pub const QVARIANT: u32     = 0x00000090;
pub const QVARIANTMAP: u32  = 0x00000008;
pub const QVARIANTLIST: u32 = 0x00000009;

pub const QSTRING: u32      = 0x0000000a;
pub const QSTRINGLIST: u32  = 0x0000000b;
pub const QBYTEARRAY: u32   = 0x0000000c;

pub const QTIME: u32        = 0x0000000f;
pub const QDATETIME: u32    = 0x00000010;
pub const USERTYPE: u32     = 0x0000007f;


// Basic types
pub const LONG: u32         = 0x00000081; // int64_t
pub const INT: u32          = 0x00000002; // int32_t
pub const SHORT: u32        = 0x00000082; // int16_t
pub const CHAR: u32         = 0x00000083; // int8_t

pub const ULONG: u32        = 0x00000084; // uint64_t
pub const UINT: u32         = 0x00000003; // uint32_t
pub const USHORT: u32       = 0x00000085; // uint16_t
pub const UCHAR: u32        = 0x00000086; // uint8_t

extern crate byteorder;
use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

use std::vec::Vec;
use std::result::Result;
use std::convert::TryInto;

use crate::util;
use crate::protocol::error::ErrorKind;
use crate::protocol::primitive::{deserialize, serialize, qread};

impl deserialize::Deserialize for bool {
    fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        if b[0] == 0 {
            return Ok((1, false))
        } else if b[0] == 1 {
            return Ok((1, true))
        } else {
            return Err(ErrorKind::BoolOutOfRange);
        };
    }
}

impl qread::QRead for bool {
    fn read<T: std::io::Read>(s: &mut T, b: &mut [u8]) -> Result<usize, ErrorKind> {
        Ok(s.read(&mut b[0..1])?)
    }
}

impl deserialize::Deserialize for u64 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        let mut rdr = Cursor::new(&b[0..8]);
        return Ok((8, rdr.read_u64::<BigEndian>()?));
    }
}

impl qread::QRead for u64 {
    fn read<T: std::io::Read>(s: &mut T, b: &mut [u8]) -> Result<usize, ErrorKind> {
        Ok(s.read(&mut b[0..8])?)
    }
}

impl deserialize::Deserialize for u32 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        let mut rdr = Cursor::new(&b[0..4]);
        return Ok((4, rdr.read_u32::<BigEndian>()?));
    }
}

impl qread::QRead for u32 {
    fn read<T: std::io::Read>(s: &mut T, b: &mut [u8]) -> Result<usize, ErrorKind> {
        Ok(s.read(&mut b[0..4])?)
    }
}

impl deserialize::Deserialize for u16 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        let mut rdr = Cursor::new(&b[0..2]);
        return Ok((2, rdr.read_u16::<BigEndian>()?));
    }
}

impl qread::QRead for u16 {
    fn read<T: std::io::Read>(s: &mut T, b: &mut [u8]) -> Result<usize, ErrorKind> {
        Ok(s.read(&mut b[0..2])?)
    }
}

impl deserialize::Deserialize for u8 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        return Ok((1, b[0]));
    }
}

impl qread::QRead for u8 {
    fn read<T: std::io::Read>(s: &mut T, b: &mut [u8]) -> Result<usize, ErrorKind> {
        Ok(s.read(&mut [b[0]])?)
    }
}

impl deserialize::Deserialize for i64 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        let mut rdr = Cursor::new(&b[0..8]);
        return Ok((8, rdr.read_i64::<BigEndian>()?));
    }
}

impl qread::QRead for i64 {
    fn read<T: std::io::Read>(s: &mut T, b: &mut [u8]) -> Result<usize, ErrorKind> {
        Ok(s.read(&mut b[0..8])?)
    }
}

impl deserialize::Deserialize for i32 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        let mut rdr = Cursor::new(&b[0..4]);
        return Ok((4, rdr.read_i32::<BigEndian>()?));
    }
}

impl qread::QRead for i32 {
    fn read<T: std::io::Read>(s: &mut T, b: &mut [u8]) -> Result<usize, ErrorKind> {
        Ok(s.read(&mut b[0..4])?)
    }
}

impl deserialize::Deserialize for i16 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        let mut rdr = Cursor::new(&b[0..2]);
        return Ok((2, rdr.read_i16::<BigEndian>()?));
    }
}

impl qread::QRead for i16 {
    fn read<T: std::io::Read>(s: &mut T, b: &mut [u8]) -> Result<usize, ErrorKind> {
        Ok(s.read(&mut b[0..2])?)
    }
}

impl deserialize::Deserialize for i8 {
    fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        return Ok((1, b[0].try_into()?));
    }
}

impl qread::QRead for i8 {
    fn read<T: std::io::Read>(s: &mut T, b: &mut [u8]) -> Result<usize, ErrorKind> {
        return Ok(s.read(&mut [b[0]])?)
    }
}



pub type String = std::string::String;
impl serialize::Serialize for String {
    fn serialize(&self) -> Result<Vec<u8>, ErrorKind> {
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
    fn serialize_utf8(&self) -> Result<Vec<u8>, ErrorKind> {
        let mut res: Vec<u8> = Vec::new();
        res.extend(self.clone().into_bytes());
        util::prepend_byte_len(&mut res);
        return Ok(res);
    }
}

impl deserialize::Deserialize for String {
    fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        let (_, len) = i32::parse(&b[0..4])?;

        let ulen = len as usize;
        let mut pos: usize = 4;
        let mut chars: Vec<u16> = Vec::new();
        loop {
            if pos >= (ulen + 4) { break; }
            let (slen, uchar) = u16::parse(&b[pos..(pos+2)])?;
            chars.push(uchar);
            pos += slen;
        }

        let res: String = String::from_utf16(&chars).unwrap();
        return Ok((pos, res));
    }
}

impl deserialize::DeserializeUTF8 for String {
    fn parse_utf8(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        use crate::protocol::primitive::deserialize::Deserialize;
        let (_, len) = i32::parse(&b[0..4])?;

        let ulen = len as usize;

        let res: String = String::from_utf8(b[4..(ulen+4)].to_vec())?;
        return Ok((ulen + 4, res));
    }
}

impl qread::QRead for String {
    fn read<T: std::io::Read>(s: &mut T, b: &mut [u8]) -> Result<usize, ErrorKind> {
        use crate::protocol::primitive::deserialize::Deserialize;

        s.read(&mut b[0..4])?;
        let (_, len) = i32::parse(&b[0..4])?;

        let ulen = len as usize;
        s.read(&mut b[4..(4+ulen)])?;

        return Ok(4 + ulen);
    }
}

pub type StringList = Vec<String>;
impl serialize::Serialize for StringList {
    fn serialize(&self) -> Result<Vec<u8>, ErrorKind> {
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
    fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        let (_, len) = i32::parse(&b[0..4])?;
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

impl qread::QRead for StringList {
    fn read<T: std::io::Read>(s: &mut T, b: &mut [u8]) -> Result<usize, ErrorKind> {
        use crate::protocol::primitive::deserialize::Deserialize;

        s.read(&mut b[0..4])?;
        let (_, len) = i32::parse(&b[0..4])?;

        let mut pos: usize = 4;
        for _ in 0..len {
            pos += String::read(s, &mut b[pos..])?;
        }

        return Ok(pos);
    }
}
