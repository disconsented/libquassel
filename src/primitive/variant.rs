use std::vec::Vec;

use failure::Error;

use log::{error, trace};

use crate::error::ProtocolError;
use crate::primitive;
use crate::primitive::StringList;
use crate::{Deserialize, DeserializeUTF8};
use crate::{Serialize, SerializeUTF8};

extern crate bytes;

use crate::primitive::{
    BufferInfo, Date, DateTime, Message, Time, VariantList, VariantMap,
};

/// Variant represents the possible types we can receive
///
/// Variant's are serizalized as the Type as a i32 and then the Type in it's own format
///
/// BufferInfo and Message are UserTypes
/// but we represent them as a native Type here.
///
/// StringUTF8 is de-/serialized as a C ByteArray.
#[allow(non_camel_case_types, dead_code)]
#[derive(Clone, Debug, std::cmp::PartialEq)]
pub enum Variant {
    Unknown,
    UserType(String, Vec<u8>),
    BufferInfo(BufferInfo),
    Message(Message),
    Time(Time),
    Date(Date),
    DateTime(DateTime),
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
            }
            Variant::VariantMap(v) => {
                res.extend(primitive::QVARIANTMAP.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.serialize()?.iter());
            }
            Variant::VariantList(v) => {
                res.extend(primitive::QVARIANTLIST.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.serialize()?.iter());
            }
            Variant::String(v) => {
                res.extend(primitive::QSTRING.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.serialize()?.iter());
            }
            Variant::StringUTF8(v) => {
                res.extend(primitive::QBYTEARRAY.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.serialize_utf8()?.iter());
            }
            Variant::StringList(v) => {
                res.extend(primitive::QSTRINGLIST.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.serialize()?.iter());
            }
            Variant::bool(v) => {
                res.extend(primitive::BOOL.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                let i = *v as i8;
                res.extend(i.to_be_bytes().iter());
            }
            Variant::u64(v) => {
                res.extend(primitive::ULONG.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            }
            Variant::u32(v) => {
                res.extend(primitive::UINT.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            }
            Variant::u16(v) => {
                res.extend(primitive::USHORT.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            }
            Variant::u8(v) => {
                res.extend(primitive::UCHAR.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            }
            Variant::i64(v) => {
                res.extend(primitive::LONG.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            }
            Variant::i32(v) => {
                res.extend(primitive::INT.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            }
            Variant::i16(v) => {
                res.extend(primitive::SHORT.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            }
            Variant::i8(v) => {
                res.extend(primitive::CHAR.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.extend(v.to_be_bytes().iter());
            }
            Variant::UserType(name, bytes) => {
                res.extend(primitive::USERTYPE.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.append(&mut name.serialize_utf8()?);
                res.extend(bytes);
            }
            Variant::BufferInfo(v) => {
                let bytes = BufferInfo::serialize(v)?;
                let user = Variant::UserType("BufferInfo".to_string(), bytes);
                Variant::serialize(&user).unwrap();
            }
            Variant::Message(v) => {
                let bytes = Message::serialize(v)?;
                let user = Variant::UserType("Message".to_string(), bytes);
                Variant::serialize(&user).unwrap();
            }
            Variant::DateTime(v) => {
                res.extend(primitive::QDATETIME.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.append(&mut v.serialize()?);
            }
            Variant::Time(v) => {
                res.extend(primitive::QTIME.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.append(&mut v.serialize()?);
            }
            Variant::Date(v) => {
                res.extend(primitive::QDATE.to_be_bytes().iter());
                res.extend(unknown.to_be_bytes().iter());
                res.append(&mut v.serialize()?);
            }
        }

        return Ok(res);
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
                trace!(target: "primitive::Variant", "Parsing Variant: VariantMap");
                let (vlen, value) = VariantMap::parse(&b[len..])?;
                return Ok((len + vlen, Variant::VariantMap(value)));
            }
            primitive::QVARIANTLIST => {
                trace!(target: "primitive::Variant", "Parsing Variant: VariantList");
                let (vlen, value) = VariantList::parse(&b[len..])?;
                return Ok((len + vlen, Variant::VariantList(value)));
            }
            primitive::QSTRING => {
                trace!(target: "primitive::Variant", "Parsing Variant: String");
                let (vlen, value) = String::parse(&b[len..])?;
                return Ok((len + vlen, Variant::String(value.clone())));
            }
            primitive::QBYTEARRAY => {
                trace!(target: "primitive::Variant", "Parsing Variant: ByteArray");
                let (vlen, value) = String::parse_utf8(&b[len..])?;
                return Ok((len + vlen, Variant::StringUTF8(value.clone())));
            }
            primitive::QSTRINGLIST => {
                trace!(target: "primitive::Variant", "Parsing Variant: StringList");
                let (vlen, value) = StringList::parse(&b[len..])?;
                return Ok((len + vlen, Variant::StringList(value.clone())));
            }
            primitive::QDATETIME => {
                trace!(target: "primitive::Variant", "Parsing Variant: Date");
                let (vlen, value) = Date::parse(&b[len..])?;
                return Ok((len + vlen, Variant::Date(value.clone())));
            }
            primitive::QDATE => {
                trace!(target: "primitive::Variant", "Parsing Variant: Date");
                let (vlen, value) = Date::parse(&b[len..])?;
                return Ok((len + vlen, Variant::Date(value.clone())));
            }
            primitive::QTIME => {
                trace!(target: "primitive::Variant", "Parsing Variant: Time");
                let (vlen, value) = Time::parse(&b[len..])?;
                return Ok((len + vlen, Variant::Time(value.clone())));
            }
            primitive::BOOL => {
                let (vlen, value) = bool::parse(&b[len..])?;
                return Ok((len + vlen, Variant::bool(value)));
            }
            primitive::ULONG => {
                let (vlen, value) = u64::parse(&b[len..])?;
                return Ok((len + vlen, Variant::u64(value)));
            }
            primitive::UINT => {
                let (vlen, value) = u32::parse(&b[len..])?;
                return Ok((len + vlen, Variant::u32(value)));
            }
            primitive::USHORT => {
                let (vlen, value) = u16::parse(&b[len..])?;
                return Ok((len + vlen, Variant::u16(value)));
            }
            primitive::UCHAR => {
                let (vlen, value) = u8::parse(&b[len..])?;
                return Ok((len + vlen, Variant::u8(value)));
            }
            primitive::LONG => {
                let (vlen, value) = i64::parse(&b[len..])?;
                return Ok((len + vlen, Variant::i64(value)));
            }
            primitive::INT => {
                let (vlen, value) = i32::parse(&b[len..])?;
                return Ok((len + vlen, Variant::i32(value)));
            }
            primitive::SHORT => {
                let (vlen, value) = i16::parse(&b[len..])?;
                return Ok((len + vlen, Variant::i16(value)));
            }
            primitive::CHAR => {
                let (vlen, value) = i8::parse(&b[len..])?;
                return Ok((len + vlen, Variant::i8(value)));
            }
            primitive::USERTYPE => {
                trace!(target: "primitive::Variant", "Parsing UserType");
                // Parse UserType name
                let (user_type_len, user_type) = String::parse_utf8(&b[len..])?;

                trace!(target: "primitive::Variant", "Parsing UserType: {:?}", user_type);

                // TODO implement all these types
                // Match Possible User Types to basic structures
                match user_type.as_str() {
                    // As VariantMap
                    "IrcUser" | "IrcChannel" | "Identity" | "NetworkInfo" | "Network::Server" => {
                        trace!(target: "primitive::Variant", "UserType is VariantMap");
                        let (vlen, value) = VariantMap::parse(&b[(len + user_type_len)..])?;
                        return Ok((len + user_type_len + vlen, Variant::VariantMap(value)));
                    }
                    // As i32
                    "BufferId" | "IdentityId" | "NetworkId" | "MsgId" => {
                        trace!(target: "primitive::Variant", "UserType is i32");

                        let (vlen, value) = i32::parse(&b[(len + user_type_len)..])?;
                        return Ok((len + user_type_len + vlen, Variant::i32(value)));
                    }
                    // As i64
                    "PeerPtr" => {
                        trace!(target: "primitive::Variant", "UserType is i64");
                        let (vlen, value) = i64::parse(&b[(len + user_type_len)..])?;
                        return Ok((len + user_type_len + vlen, Variant::i64(value)));
                    }
                    "BufferInfo" => {
                        trace!(target: "primitive::Variant", "UserType is BufferInfo");
                        let (vlen, value) = BufferInfo::parse(&b[(len + user_type_len)..])?;
                        return Ok((len + user_type_len + vlen, Variant::BufferInfo(value)));
                    }
                    "Message" => {
                        trace!(target: "primitive::Variant", "UserType is Message");
                        let (vlen, value) = Message::parse(&b[(len + user_type_len)..])?;
                        return Ok((len + user_type_len + vlen, Variant::Message(value)));
                    }
                    _ => unimplemented!(),
                }
            }
            err => {
                error!(target: "parser", "UnknownVariant: {:x?}", err);
                bail!(ProtocolError::UnknownVariant);
            }
        }
    }
}
