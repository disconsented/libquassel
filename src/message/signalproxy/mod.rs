use crate::{Deserialize, Serialize};

mod heartbeat;
mod initdata;
mod initrequest;
pub mod objects;
mod rpccall;
mod syncmessage;

pub use heartbeat::*;
pub use initdata::*;
pub use initrequest::*;
pub use rpccall::*;
pub use syncmessage::*;

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

// impl Message {
//     fn act(&self) {
//         match &self {
//             Message::SyncMessage(value) => value.serialize(),
//             Message::RpcCall(value) => value.serialize(),
//             Message::InitRequest(value) => value.serialize(),
//             Message::InitData(value) => value.serialize(),
//             Message::HeartBeat(value) => value.serialize(),
//             Message::HeartBeatReply(value) => value.serialize(),
//         }
//     }
// }

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
