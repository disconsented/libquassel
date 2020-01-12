use std::vec::Vec;
use std::convert::TryInto;
use std::collections::HashMap;

use crate::types::{Serialize, Deserialize};
use crate::types::basic;
use crate::types::basic::*;
use crate::types::basic::{String,StringList};

pub type VariantMap = HashMap<String, Variant>;
impl Serialize for VariantMap {
    fn serialize<'a>(&'a self) -> Vec<u8> {
        let len: i32 = self.len().try_into().unwrap();
        let mut res: Vec<u8> = Vec::new();

        res.extend(len.to_be_bytes().iter());
        for (k, v) in self {
            res.extend(k.serialize().iter());
            res.extend(v.serialize().iter());
        }

        return res;
    }
}

impl Deserialize for VariantMap {
    fn parse(&mut self, b: &[u8]) -> usize {
        let mut len: i32 = 0;
        len.parse(&b[0..3]);

        let mut pos = 4;
        let map = VariantMap::new();
        for _ in 0..len {
            let mut name: String = String::new();
            name.parse(&b[(pos)..(pos+2)]);

            let mut value: Variant = Variant::Unknown;
            let res = value.parse(&b[(pos+3)..]);

            pos = pos + 2 + res;
        }

        *self = map;
        return pos;
    }
}

pub type VariantList = Vec<Variant>;
impl Serialize for VariantList {
    fn serialize(&self) -> Vec<u8> {
        let len: i32 = self.len().try_into().unwrap();
        let mut res: Vec<u8> = Vec::new();

        res.extend(len.to_be_bytes().iter());
        for v in self {
            res.extend(v.serialize().iter());
        }

        return res;
    }
}

impl Deserialize for VariantList {
    fn parse(&mut self, b: &[u8]) -> usize {
        let len: i32 = self.len().try_into().unwrap();
        let res: VariantList = VariantList::new();

        let mut pos: usize = 0;
        for _ in 0..len {
            let mut val: Variant = Variant::Unknown;
            pos = pos + val.parse(&b[pos..]);
        }

        *self = res;
        return pos;
    }
}

#[allow(non_camel_case_types, dead_code)]
#[derive(Clone)]
pub enum Variant {
    Unknown,
    VariantMap(VariantMap),
    VariantList(VariantList),
    String(String),
    StringList(StringList),
    bool(bool),
    u64(u64),
    u32(u32),
    u16(u16),
    u8(u8),
    i64(i64),
    i32(i32),
    i16(i16),
    i8(i8),
}

impl Serialize for Variant {
    fn serialize(&self) -> Vec<u8> {
        let unknown: u32 = 0x00000000;
        let mut res: Vec<u8> = Vec::new();

        match self {
            Variant::Unknown => {
               return res;
            },
            Variant::VariantMap(v) => {
                res.extend(basic::QVARIANTMAP.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.serialize().iter());
            },
            Variant::VariantList(v) => {
                res.extend(basic::QVARIANTLIST.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.serialize().iter());
            },
            Variant::String(v) => {
                res.extend(basic::QSTRING.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.serialize().iter());
            },
            Variant::StringList(v) => {
                res.extend(basic::QSTRINGLIST.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.serialize().iter());
            },
            Variant::bool(v) => {
                res.extend(basic::BOOL.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                let i = *v as i8;
                res.extend(i.to_be_bytes().iter());
            },
            Variant::u64(v) => {
                res.extend(basic::ULONG.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
            Variant::u32(v) => {
                res.extend(basic::UINT.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
            Variant::u16(v) => {
                res.extend(basic::USHORT.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
            Variant::u8(v) => {
                res.extend(basic::UCHAR.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
            Variant::i64(v) => {
                res.extend(basic::LONG.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
            Variant::i32(v) => {
                res.extend(basic::INT.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
            Variant::i16(v) => {
                res.extend(basic::SHORT.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
            Variant::i8(v) => {
                res.extend(basic::CHAR.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
        }

        return res
    }
}

impl Deserialize for Variant {
    fn parse(&mut self, b: &[u8]) -> usize {
        let mut qtype: i32 = 0;
        qtype.parse(&b[0..6]);
        let qtype: u32 = qtype as u32;

        #[allow(unused_variables)]
        let unknown: u8 = b[7];

        match qtype {
            QVARIANTMAP => {
                let mut value: VariantMap = VariantMap::new();

                let len = value.parse(&b[8..]);
                *self = Variant::VariantMap(value.clone());

                return len;
            },
            QVARIANTLIST => {
                let mut value: VariantList = VariantList::new();

                let len = value.parse(&b[8..]);
                *self = Variant::VariantList(value.clone());

                return len;
            },
            QSTRING => {
                let mut value: String = String::new();

                let len = value.parse(&b[8..]);
                *self = Variant::String(value.clone());

                return len;
            },
            QSTRINGLIST => {
                let mut value: StringList = StringList::new();

                let len = value.parse(&b[8..]);
                *self = Variant::StringList(value.clone());

                return len;
            },
            BOOL => {
                let mut value: bool = false;

                let len = value.parse(&b[8..]);
                *self = Variant::bool(value);

                return len;
            },
            ULONG => {
                let mut value: u64 = 0;

                let len = value.parse(&b[8..]);
                *self = Variant::u64(value);

                return len;
            },
            UINT => {
                let mut value: u32 = 0;

                let len = value.parse(&b[8..]);
                *self = Variant::u32(value);

                return len;
            },
            USHORT => {
                let mut value: u16 = 0;

                let len = value.parse(&b[8..]);
                *self = Variant::u16(value);

                return len;
            },
            UCHAR => {
                let mut value: u8 = 0;

                let len = value.parse(&b[8..]);
                *self = Variant::u8(value);

                return len;
            },
            LONG => {
                let mut value: i64 = 0;

                let len = value.parse(&b[8..]);
                *self = Variant::i64(value);

                return len;
            },
            INT => {
                let mut value: i32 = 0;

                let len = value.parse(&b[8..]);
                *self = Variant::i32(value);

                return len;
            },
            SHORT => {
                let mut value: i16 = 0;

                let len = value.parse(&b[8..]);
                *self = Variant::i16(value);

                return len;
            },
            CHAR => {
                let mut value: i8 = 0;

                let len = value.parse(&b[8..]);
                *self = Variant::i8(value);

                return len;
            },
            _ => {
                return 0;
            }
        }
    }
}
