use std::vec::Vec;

use failure::Error;

use crate::{Deserialize, DeserializeUTF8};
use crate::{Serialize, SerializeUTF8};

extern crate bytes;

/// The BufferInfo struct represents a BufferInfo as received in IRC
///
/// BufferInfo is, like all other struct based types, serialized sequentially.
#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct BufferInfo {
    /// a unique, sequential id for the buffer
    pub id: i32,
    /// NetworkId of the network the buffer belongs to
    pub network_id: i32,
    /// The Type of the Buffer
    pub buffer_type: BufferType,
    /// BufferName as displayed to the user
    pub name: String,
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

        // There are 4 additional undocumented Bytes in the BufferInfo
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

/// The Type of the Buffer
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
