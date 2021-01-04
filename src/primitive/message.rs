use std::vec::Vec;

use failure::Error;

use crate::{Deserialize, DeserializeUTF8};
use crate::{Serialize, SerializeUTF8};

use crate::primitive::BufferInfo;

extern crate bytes;

/// The Message struct represents a Message as received in IRC
///
/// Messages are, like all other struct based types, serialized sequentially.
#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct Message {
    /// The unique, sequential id for the message
    /// i32 by default i64 if long-message-id features is enabled
    #[cfg(feature = "long-message-id")]
    #[cfg_attr(docsrs, doc(cfg(feature = "long-message-id")))]
    pub msg_id: i64,
    #[cfg(not(feature = "long-message-id"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "long-message-id"))))]
    pub msg_id: i32,
    /// The timestamp of the message in UNIX time (32-bit, seconds, 64-bit if long-time feature enabled)
    #[cfg(feature = "long-time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "long-time")))]
    pub timestamp: i64,
    #[cfg(not(feature = "long-time"))]
    #[cfg_attr(docsrs, doc(cfg(not(feature = "long-time"))))]
    pub timestamp: i32,
    /// The message type as it's own type serialized as i32
    pub msg_type: MessageType,
    /// The flags
    pub flags: i8,
    /// The buffer the message belongs to, usually everything but BufferId is set to NULL
    pub buffer: BufferInfo,
    /// The sender as nick!ident@host
    pub sender: String,
    /// The prefix modes of the sender.
    #[cfg(feature = "sender-prefixes")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sender-prefixes")))]
    pub sender_prefixes: String,
    /// The realName of the sender
    #[cfg(feature = "rich-messages")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rich-messages")))]
    pub real_name: String,
    /// The avatarUrl of the sender, if available
    #[cfg(feature = "rich-messages")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rich-messages")))]
    pub avatar_url: String,
    /// The message content, already stripped from CTCP formatting, but containing mIRC format codes
    pub content: String,
}

impl Serialize for Message {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut values: Vec<u8> = Vec::new();

        #[cfg(feature = "long-message-id")]
        values.append(&mut i64::serialize(&self.msg_id)?);
        #[cfg(not(feature = "long-message-id"))]
        values.append(&mut i32::serialize(&self.msg_id)?);

        #[cfg(feature = "long-time")]
        values.append(&mut i64::serialize(&self.timestamp)?);
        #[cfg(not(feature = "long-time"))]
        values.append(&mut i32::serialize(&(self.timestamp as i32))?);

        values.append(&mut i32::serialize(&(self.msg_type as i32))?);
        values.append(&mut i8::serialize(&(self.flags as i8))?);
        values.append(&mut BufferInfo::serialize(&self.buffer)?);
        values.append(&mut String::serialize_utf8(&self.sender)?);

        #[cfg(feature = "sender-prefixes")]
        values.append(&mut String::serialize_utf8(&self.sender_prefixes)?);

        #[cfg(feature = "rich-messages")]
        {
            values.append(&mut String::serialize_utf8(&self.real_name)?);
            values.append(&mut String::serialize_utf8(&self.avatar_url)?);
        }

        values.append(&mut String::serialize_utf8(&self.content)?);

        return Ok(values);
    }
}

impl Deserialize for Message {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let mut pos = 0;
        #[cfg(feature = "long-message-id")]
        let (parsed, msg_id) = i64::parse(&b[pos..])?;
        #[cfg(not(feature = "long-message-id"))]
        let (parsed, msg_id) = i32::parse(&b[pos..])?;
        pos += parsed;

        // TODO LONGMESSAGES feature
        let timestamp;

        #[cfg(feature = "long-time")]
        {
            let (parsed, temp_timestamp) = i64::parse(&b[pos..])?;
            pos += parsed;
            timestamp = temp_timestamp;
        }

        #[cfg(not(feature = "long-time"))]
        {
            let (parsed, temp_timestamp) = i32::parse(&b[pos..])?;
            pos += parsed;
            timestamp = temp_timestamp;
        }

        let (parsed, msg_type) = i32::parse(&b[pos..])?;
        pos += parsed;
        let (parsed, flags) = i8::parse(&b[pos..])?;
        pos += parsed;
        let (parsed, buffer) = BufferInfo::parse(&b[pos..])?;
        pos += parsed;
        let (parsed, sender) = String::parse_utf8(&b[pos..])?;
        pos += parsed;

        #[cfg(feature = "sender-prefixes")]
        let sender_prefixes: String;
        #[cfg(feature = "sender-prefixes")]
        {
            let (parsed, temp) = String::parse_utf8(&b[pos..])?;
            sender_prefixes = temp;
            pos += parsed;
        }

        #[cfg(feature = "rich-messages")]
        let real_name: String;
        #[cfg(feature = "rich-messages")]
        let avatar_url: String;
        #[cfg(feature = "rich-messages")]
        {
            let (parsed, temp) = String::parse_utf8(&b[pos..])?;
            real_name = temp;
            pos += parsed;

            let (parsed, temp) = String::parse_utf8(&b[pos..])?;
            avatar_url = temp;
            pos += parsed;
        }

        let (parsed, content) = String::parse_utf8(&b[pos..])?;
        pos += parsed;

        return Ok((
            pos,
            Self {
                msg_id,
                timestamp,
                msg_type: MessageType::from(msg_type),
                flags,
                buffer,
                sender,
                #[cfg(feature = "sender-prefixes")]
                sender_prefixes,
                #[cfg(feature = "rich-messages")]
                real_name,
                #[cfg(feature = "rich-messages")]
                avatar_url,
                content,
            },
        ));
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, std::cmp::PartialEq)]
pub enum MessageType {
    Plain = 0x00000001,
    Notice = 0x00000002,
    Action = 0x00000004,
    Nick = 0x00000008,
    Mode = 0x00000010,
    Join = 0x00000020,
    Part = 0x00000040,
    Quit = 0x00000080,
    Kick = 0x00000100,
    Kill = 0x00000200,
    Server = 0x00000400,
    Info = 0x00000800,
    Error = 0x00001000,
    DayChange = 0x00002000,
    Topic = 0x00004000,
    NetsplitJoin = 0x00008000,
    NetsplitQuit = 0x00010000,
    Invite = 0x00020000,
    Markerline = 0x00040000,
}

impl From<i32> for MessageType {
    fn from(val: i32) -> Self {
        match val {
            0x00000001 => MessageType::Plain,
            0x00000002 => MessageType::Notice,
            0x00000004 => MessageType::Action,
            0x00000008 => MessageType::Nick,
            0x00000010 => MessageType::Mode,
            0x00000020 => MessageType::Join,
            0x00000040 => MessageType::Part,
            0x00000080 => MessageType::Quit,
            0x00000100 => MessageType::Kick,
            0x00000200 => MessageType::Kill,
            0x00000400 => MessageType::Server,
            0x00000800 => MessageType::Info,
            0x00001000 => MessageType::Error,
            0x00002000 => MessageType::DayChange,
            0x00004000 => MessageType::Topic,
            0x00008000 => MessageType::NetsplitJoin,
            0x00010000 => MessageType::NetsplitQuit,
            0x00020000 => MessageType::Invite,
            0x00040000 => MessageType::Markerline,
            _ => unimplemented!(),
        }
    }
}
