use crate::primitive::{DateTime, Variant, VariantList};
use crate::{Deserialize, Serialize};

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub enum Message {
    /// Bidirectional
    SyncMessage(SyncMessage),
    /// Bidirectional
    RpcCall(RpcCall),
    InitRequest(InitRequest),
    InitData(InitData),
    /// Bidirectional
    HeartBeat(HeartBeat),
    /// Bidirectional
    HeartBeatReply(HeartBeatReply),
}

impl Serialize for Message {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, failure::Error> {
        match &self {
            Message::SyncMessage(value) => value.serialize(),
            Message::RpcCall(value) => value.serialize(),
            Message::InitRequest(value) => value.serialize(),
            Message::InitData(value) => value.serialize(),
            Message::HeartBeat(value) => value.serialize(),
            Message::HeartBeatReply(value) => value.serialize(),
        }
    }
}

impl Deserialize for Message {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), failure::Error> {
        let (_, message_type) = i32::parse(&b[9..13])?;

        match MessageType::from(message_type) {
            MessageType::SyncMessage => {
                let (size, res) = SyncMessage::parse(&b)?;

                Ok((size, Message::SyncMessage(res)))
            }
            MessageType::RpcCall => {
                let (size, res) = RpcCall::parse(&b)?;

                Ok((size, Message::RpcCall(res)))
            }
            MessageType::InitRequest => {
                let (size, res) = InitRequest::parse(&b)?;

                Ok((size, Message::InitRequest(res)))
            }
            MessageType::InitData => {
                let (size, res) = InitData::parse(&b)?;

                Ok((size, Message::InitData(res)))
            }
            MessageType::HeartBeat => {
                let (size, res) = HeartBeat::parse(&b)?;

                Ok((size, Message::HeartBeat(res)))
            }
            MessageType::HeartBeatReply => {
                let (size, res) = HeartBeatReply::parse(&b)?;

                Ok((size, Message::HeartBeatReply(res)))
            }
        }
    }
}

/// Type of an SignalProxy Message
/// The first element in the VariantList that is received
#[repr(i32)]
#[derive(Copy, Clone, Debug, std::cmp::PartialEq)]
pub enum MessageType {
    /// Bidirectional
    SyncMessage = 0x00000001,
    /// Bidirectional
    RpcCall = 0x00000002,
    InitRequest = 0x00000003,
    InitData = 0x00000004,
    /// Bidirectional
    HeartBeat = 0x00000005,
    /// Bidirectional
    HeartBeatReply = 0x00000006,
}

impl From<i32> for MessageType {
    fn from(val: i32) -> Self {
        match val {
            0x00000001 => MessageType::SyncMessage,
            0x00000002 => MessageType::RpcCall,
            0x00000003 => MessageType::InitRequest,
            0x00000004 => MessageType::InitData,
            0x00000005 => MessageType::HeartBeat,
            0x00000006 => MessageType::HeartBeatReply,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct SyncMessage {
    class_name: String,
    object_name: String,
    slot_name: String,
    params: VariantList,
}

impl Serialize for SyncMessage {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, failure::Error> {
        let mut res = VariantList::new();

        res.push(Variant::i32(MessageType::SyncMessage as i32));
        res.push(Variant::StringUTF8(self.class_name.clone()));
        res.push(Variant::StringUTF8(self.object_name.clone()));
        res.push(Variant::StringUTF8(self.slot_name.clone()));

        res.append(&mut self.params.clone());

        res.serialize()
    }
}

impl Deserialize for SyncMessage {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), failure::Error> {
        let (size, mut res) = VariantList::parse(&b)?;

        res.remove(0);

        Ok((
            size,
            Self {
                class_name: match_variant!(res.remove(0), Variant::StringUTF8),
                object_name: match_variant!(res.remove(0), Variant::StringUTF8),
                slot_name: match_variant!(res.remove(0), Variant::StringUTF8),
                params: res,
            },
        ))
    }
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct RpcCall {
    slot_name: String,
    params: VariantList,
}

impl Serialize for RpcCall {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, failure::Error> {
        let mut res = VariantList::new();

        res.push(Variant::i32(MessageType::RpcCall as i32));
        res.push(Variant::StringUTF8(self.slot_name.clone()));

        res.append(&mut self.params.clone());

        res.serialize()
    }
}

impl Deserialize for RpcCall {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), failure::Error> {
        let (size, mut res) = VariantList::parse(&b)?;

        res.remove(0);

        Ok((
            size,
            Self {
                slot_name: match_variant!(res.remove(0), Variant::StringUTF8),
                params: res,
            },
        ))
    }
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct InitRequest {
    class_name: String,
    object_name: String,
}

impl Serialize for InitRequest {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, failure::Error> {
        let mut res = VariantList::new();

        res.push(Variant::i32(MessageType::InitRequest as i32));
        res.push(Variant::StringUTF8(self.class_name.clone()));
        res.push(Variant::StringUTF8(self.object_name.clone()));

        res.serialize()
    }
}

impl Deserialize for InitRequest {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), failure::Error> {
        let (size, mut res) = VariantList::parse(&b)?;

        res.remove(0);

        Ok((
            size,
            Self {
                class_name: match_variant!(res.remove(0), Variant::StringUTF8),
                object_name: match_variant!(res.remove(0), Variant::StringUTF8),
            },
        ))
    }
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct InitData {
    class_name: String,
    object_name: String,
    init_data: VariantList,
}

impl Serialize for InitData {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, failure::Error> {
        let mut res = VariantList::new();

        res.push(Variant::i32(MessageType::InitData as i32));
        res.push(Variant::StringUTF8(self.class_name.clone()));
        res.push(Variant::StringUTF8(self.object_name.clone()));

        res.append(&mut self.init_data.clone());

        res.serialize()
    }
}

impl Deserialize for InitData {
    fn parse(b: &[std::primitive::u8]) -> Result<(std::primitive::usize, Self), failure::Error> {
        let (size, mut res) = VariantList::parse(&b)?;

        res.remove(0);

        Ok((
            size,
            Self {
                class_name: match_variant!(res.remove(0), Variant::StringUTF8),
                object_name: match_variant!(res.remove(0), Variant::StringUTF8),
                init_data: res,
            },
        ))
    }
}

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
