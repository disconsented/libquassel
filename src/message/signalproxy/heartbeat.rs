use crate::message::MessageType;
use crate::primitive::{DateTime, Variant, VariantList};
use crate::{Deserialize, Serialize};

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct HeartBeat {
    timestamp: DateTime,
}

impl Serialize for HeartBeat {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, failure::Error> {
        let mut res = VariantList::new();

        res.push(Variant::i32(MessageType::HeartBeat as i32));
        res.push(Variant::DateTime(self.timestamp.clone()));

        res.serialize()
    }
}

impl Deserialize for HeartBeat {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), failure::Error> {
        let (size, mut res) = VariantList::parse(&b)?;

        res.remove(0);

        Ok((
            size,
            Self {
                timestamp: match_variant!(res.remove(0), Variant::DateTime),
            },
        ))
    }
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct HeartBeatReply {
    timestamp: DateTime,
}

impl Serialize for HeartBeatReply {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, failure::Error> {
        let mut res = VariantList::new();

        res.push(Variant::i32(MessageType::HeartBeatReply as i32));
        res.push(Variant::DateTime(self.timestamp.clone()));

        res.serialize()
    }
}

impl Deserialize for HeartBeatReply {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), failure::Error> {
        let (size, mut res) = VariantList::parse(&b)?;

        res.remove(0);

        Ok((
            size,
            Self {
                timestamp: match_variant!(res.remove(0), Variant::DateTime),
            },
        ))
    }
}
