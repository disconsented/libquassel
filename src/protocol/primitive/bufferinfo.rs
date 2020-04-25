use std::vec::Vec;

use failure::Error;

use crate::protocol::primitive::deserialize::{Deserialize, DeserializeUTF8};
use crate::protocol::primitive::serialize::{Serialize, SerializeUTF8};
use crate::protocol::primitive::String;

extern crate bytes;

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct BufferInfo {
    pub id: i32,         // a unique, sequential id for the buffer
    pub network_id: i32, // NetworkId of the network the buffer belongs to
    pub buffer_type: BufferType,
    pub name: String, // BufferName as displayed to the user
}

impl Serialize for BufferInfo {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut values: Vec<u8> = Vec::new();

        values.append(&mut i32::serialize(&self.id)?);
        values.append(&mut i32::serialize(&self.network_id)?);
        values.append(&mut i16::serialize(&(self.buffer_type as i16))?);
        values.append(&mut vec![0, 0, 0, 0]);
        values.append(&mut String::serialize_utf8(&self.name)?);

        Ok(values)
    }
}

impl Deserialize for BufferInfo {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (_, id) = i32::parse(&b[0..4])?;
        let (_, network_id) = i32::parse(&b[4..8])?;
        let (_, buffer_type) = i16::parse(&b[8..10])?;

        // There are 4 additional undocumted Bytes in the BufferInfo
        // so we start at byte 14
        let (size, name) = String::parse_utf8(&b[14..])?;

        return Ok((
            14 + size,
            Self {
                id,
                network_id,
                buffer_type: BufferType::from(buffer_type),
                name,
            },
        ));
    }
}

#[repr(i16)]
#[derive(Copy, Clone, Debug, std::cmp::PartialEq)]
pub enum BufferType {
    Status = 0x01,
    Channel = 0x02,
    Query = 0x04,
    Group = 0x08,
}

impl From<i16> for BufferType {
    fn from(value: i16) -> Self {
        match value {
            0x01 => return Self::Status,
            0x02 => return Self::Channel,
            0x04 => return Self::Query,
            0x08 => return Self::Group,
            _ => unimplemented!(),
        }
    }
}
