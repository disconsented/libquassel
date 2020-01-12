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
use byteorder::{ByteOrder, BigEndian};

use std::vec::Vec;
use std::convert::TryInto;

use crate::types;

impl types::Deserialize for bool {
    fn parse(&mut self, b: &[u8]) -> usize {
        if b[0] == 0 { *self = false } else { *self = true };
        return 1;
    }
}

impl types::Deserialize for u64 {
    fn parse(&mut self, b: &[u8]) -> usize {
        *self = BigEndian::read_u64(b);
        return 8;
    }
}
impl types::Deserialize for u32 {
    fn parse(&mut self, b: &[u8]) -> usize {
        // self = &rdr.read_u32::<BigEndian>().unwrap()
        *self = BigEndian::read_u32(b);
        return 4;
    }
}
impl types::Deserialize for u16 {
    fn parse(&mut self, b: &[u8]) -> usize {
        *self = BigEndian::read_u16(b);
        return 2;
    }
}
impl types::Deserialize for u8 {
    fn parse(&mut self, b: &[u8]) -> usize {
        *self = b[0];
        return 1;
    }
}

impl types::Deserialize for i64 {
    fn parse(&mut self, b: &[u8]) -> usize {
        *self = BigEndian::read_i64(b);
        return 8;
    }
}
impl types::Deserialize for i32 {
    fn parse(&mut self, b: &[u8]) -> usize {
        *self = BigEndian::read_i32(b);
        return 4;
    }
}
impl types::Deserialize for i16 {
    fn parse(&mut self, b: &[u8]) -> usize {
        *self = BigEndian::read_i16(b);
        return 2;
    }
}
impl types::Deserialize for i8 {
    fn parse(&mut self, b: &[u8]) -> usize {
        *self = b[0].try_into().unwrap();
        return 1;
    }
}


pub type String = std::string::String;
impl types::Serialize for String {
    fn serialize(&self) -> Vec<u8> {
        let len: i32 = self.len().try_into().unwrap();
        let mut res: Vec<u8> = Vec::new();

        res.extend(len.to_be_bytes().iter());
        res.extend(self.parse::<u16>().unwrap().to_be_bytes().iter());

        return res;
    }
}

impl types::Deserialize for String {
    fn parse(&mut self, b: &[u8]) -> usize {
        let mut len: i32 = 0;
        len.parse(&b[0..4]);
        let ulen: usize = len as usize;
        *self = BigEndian::read_u16(&b[(5)..(5+ulen)]).to_string();
        b.len()
    }
}

pub type StringList = Vec<String>;
impl types::Serialize for StringList {
    fn serialize(&self) -> Vec<u8> {
        let len: i32 = self.len().try_into().unwrap();
        let mut res: Vec<u8> = Vec::new();

        res.extend(len.to_be_bytes().iter());
        for x in self {
            res.extend(x.parse::<u16>().unwrap().to_be_bytes().iter());
        }

        return res;
    }
}

impl types::Deserialize for StringList {
    fn parse(&mut self, b: &[u8]) -> usize {
        let len: i32 = self.len().try_into().unwrap();
        let res: StringList = StringList::new();

        let mut pos: usize = 0;
        for _ in 0..len {
            let mut val: String = String::new();
            pos = pos + val.parse(&b[pos..]);
        }

        *self = res;
        return pos;
    }
}
