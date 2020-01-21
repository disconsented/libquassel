use std::io::Read;
use std::vec::Vec;
use std::convert::TryInto;
use std::collections::HashMap;

use failure::Error;

use crate::util;
use crate::protocol::primitive::serialize::{Serialize, SerializeUTF8};
use crate::protocol::primitive::deserialize::{Deserialize, DeserializeUTF8};
use crate::protocol::primitive::qread::QRead;
use crate::protocol::primitive::{String,StringList};
use crate::protocol::error::ProtocolError;
use crate::protocol::primitive;

pub type VariantMap = HashMap<String, Variant>;

impl Serialize for VariantMap {
    fn serialize<'a>(&'a self) -> Result<Vec<u8>, Error> {
        let mut res: Vec<u8> = Vec::new();

        for (k, v) in self {
            res.extend(k.serialize()?);
            res.extend(v.serialize()?);
        }

        let len: i32 = self.len().try_into()?;
        util::insert_bytes(0, &mut res, &mut len.to_be_bytes());

        return Ok(res);
    }
}

impl Deserialize for VariantMap {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (_, len) = i32::parse(&b[0..4])?;

        let mut pos = 4;
        let mut map = VariantMap::new();
        for _ in 0..len {
            let (nlen, name) = String::parse(&b[(pos)..])?;
            pos += nlen;

            let (vlen, value) = Variant::parse(&b[(pos)..])?;
            pos += vlen;

            map.insert(name, value);
        }

        return Ok((pos, map));
    }
}

impl QRead for VariantMap {
    fn read<T: Read>(s: &mut T, b: &mut [u8]) -> Result<usize, Error> {
        s.read(&mut b[0..4])?;


        let (_, len) = i32::parse(&b[0..4])?;

        let mut pos = 4;
        for _ in 0..len {
            pos += String::read(s, &mut b[pos..])?;
            pos += Variant::read(s, &mut b[(pos+3..)])?;
        }

        return Ok(pos);
    }
}

/////////////////////////////////////////////////////////////////////////
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

        let mut res: VariantList = VariantList::new();
        let mut pos: usize = 4;
        for _ in 0..len {
            let (vlen, val) = Variant::parse(&b[pos..])?;
            res.push(val);
            pos += vlen;
        }

        return Ok((pos, res));
    }
}

impl QRead for VariantList {
    fn read<T: Read>(s: &mut T, b: &mut [u8]) -> Result<usize, Error> {
        s.read(&mut b[0..4])?;

        let (_, len) = i32::parse(&b[0..4])?;

        let mut pos = 4;
        for _ in 0..len {
            pos += Variant::read(s, &mut b[(pos+3..)])?;
        }

        return Ok(pos);
    }
}

/////////////////////////////////////////////////////////////////////////
#[allow(non_camel_case_types, dead_code)]
#[derive(Clone, Debug, std::cmp::PartialEq)]
pub enum Variant {
    Unknown,
    VariantMap(VariantMap),
    VariantList(VariantList),
    String(String),
    StringUTF8(String),
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
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let unknown: u8 = 0x00;
        let mut res: Vec<u8> = Vec::new();

        match self {
            Variant::Unknown => {
               bail!(ProtocolError::UnknownVariant);
            },
            Variant::VariantMap(v) => {
                res.extend(primitive::QVARIANTMAP.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.serialize()?.iter());
            },
            Variant::VariantList(v) => {
                res.extend(primitive::QVARIANTLIST.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.serialize()?.iter());
            },
            Variant::String(v) => {
                res.extend(primitive::QSTRING.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.serialize()?.iter());
            },
            Variant::StringUTF8(v) => {
                res.extend(primitive::QBYTEARRAY.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.serialize_utf8()?.iter());
            },
            Variant::StringList(v) => {
                res.extend(primitive::QSTRINGLIST.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.serialize()?.iter());
            },
            Variant::bool(v) => {
                res.extend(primitive::BOOL.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                let i = *v as i8;
                res.extend(i.to_be_bytes().iter());
            },
            Variant::u64(v) => {
                res.extend(primitive::ULONG.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
            Variant::u32(v) => {
                res.extend(primitive::UINT.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
            Variant::u16(v) => {
                res.extend(primitive::USHORT.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
            Variant::u8(v) => {
                res.extend(primitive::UCHAR.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
            Variant::i64(v) => {
                res.extend(primitive::LONG.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
            Variant::i32(v) => {
                res.extend(primitive::INT.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
            Variant::i16(v) => {
                res.extend(primitive::SHORT.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
            Variant::i8(v) => {
                res.extend(primitive::CHAR.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            },
        }

        return Ok(res)
    }
}

impl Deserialize for Variant {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (_, qtype) = i32::parse(&b[0..4])?;
        let qtype = qtype as u32;

        #[allow(unused_variables)]
        let unknown: u8 = b[4];

        let len = 5;
        match qtype {
            primitive::QVARIANTMAP => {
                let (vlen, value) = VariantMap::parse(&b[len..])?;
                return Ok((len+vlen, Variant::VariantMap(value)));
            },
            primitive::QVARIANTLIST => {
                let (vlen, value) = VariantList::parse(&b[len..])?;
                return Ok((len+vlen, Variant::VariantList(value)));
            },
            primitive::QSTRING => {
                let (vlen, value) = String::parse(&b[len..])?;
                return Ok((len+vlen, Variant::String(value.clone())));
            },
            primitive::QBYTEARRAY => {
                let (vlen, value) = String::parse_utf8(&b[len..])?;
                return Ok((len+vlen, Variant::StringUTF8(value.clone())));
            },
            primitive::QSTRINGLIST => {
                let (vlen, value) = StringList::parse(&b[len..])?;
                return Ok((len+vlen, Variant::StringList(value.clone())));
            },
            primitive::BOOL => {
                let (vlen, value) = bool::parse(&b[len..])?;
                return Ok((len+vlen, Variant::bool(value)));
            },
            primitive::ULONG => {
                let (vlen, value) = u64::parse(&b[len..])?;
                return Ok((len+vlen, Variant::u64(value)));
            },
            primitive::UINT => {
                let (vlen, value) = u32::parse(&b[len..])?;
                return Ok((len+vlen, Variant::u32(value)));
            },
            primitive::USHORT => {
                let (vlen, value) = u16::parse(&b[len..])?;
                return Ok((len+vlen, Variant::u16(value)));
            },
            primitive::UCHAR => {
                let (vlen, value) = u8::parse(&b[len..])?;
                return Ok((len+vlen, Variant::u8(value)));
            },
            primitive::LONG => {
                let (vlen, value) = i64::parse(&b[len..])?;
                return Ok((len+vlen, Variant::i64(value)));
            },
            primitive::INT => {
                let (vlen, value) = i32::parse(&b[len..])?;
                return Ok((len+vlen, Variant::i32(value)));
            },
            primitive::SHORT => {
                let (vlen, value) = i16::parse(&b[len..])?;
                return Ok((len+vlen, Variant::i16(value)));
            },
            primitive::CHAR => {
                let (vlen, value) = i8::parse(&b[len..])?;
                return Ok((len+vlen, Variant::i8(value)));
            },
            _ => {
                bail!(ProtocolError::UnknownVariant);
            }
        }
    }
}

impl QRead for Variant {
    fn read<T: Read>(s: &mut T, b: &mut [u8]) -> Result<usize, Error> {

        s.read(&mut b[0..4])?;
        let (_, qtype) = i32::parse(&b[0..4])?;
        let qtype = qtype as u32;

        s.read(&mut [b[4]])?;

        let mut len = 5;
        match qtype {
            primitive::QVARIANTMAP  => len += VariantMap::read(s, &mut b[len..])?,
            primitive::QVARIANTLIST => len += VariantList::read(s, &mut b[len..])?,
            primitive::QSTRING      => len += String::read(s, &mut b[len..])?,
            primitive::QBYTEARRAY   => len += String::read(s, &mut b[len..])?,
            primitive::QSTRINGLIST  => len += StringList::read(s, &mut b[len..])?,
            primitive::BOOL         => len += bool::read(s, &mut b[len..])?,
            primitive::ULONG        => len += u64::read(s, &mut b[len..])?,
            primitive::UINT         => len += u32::read(s, &mut b[len..])?,
            primitive::USHORT       => len += u16::read(s, &mut b[len..])?,
            primitive::UCHAR        => len += u8::read(s, &mut b[len..])?,
            primitive::LONG         => len += i64::read(s, &mut b[len..])?,
            primitive::INT          => len += i32::read(s, &mut b[len..])?,
            primitive::SHORT        => len += i16::read(s, &mut b[len..])?,
            primitive::CHAR         => len += i8::read(s, &mut b[len..])?,
            _ => bail!(ProtocolError::UnknownVariant)
        }

        return Ok(len);
    }
}
