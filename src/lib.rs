#![feature(array_value_iter)]
#![feature(external_doc)]
#![feature(doc_cfg)]
#![doc(include = "../README.md")]
#[cfg_attr(docsrs, feature(doc_cfg))]
#[macro_use]
mod util;

#[macro_use]
extern crate failure;

/// Quassel Structures for serialization and deserialization
pub mod message;

/// Quassels QT based primitive types that make up the more complex messages
pub mod primitive;

pub mod session;

#[allow(dead_code)]
/// Error Types
pub mod error;

#[allow(unused_variables, dead_code)]
#[cfg(feature = "framing")]
#[cfg_attr(docsrs, doc(cfg(feature = "framing")))]
/// Framing impl to be used with [`tokio_util::codec::Framed`]
pub mod frame;

use failure::Error;

/// Serialization of types and structs to the quassel byteprotocol
pub trait Serialize {
    fn serialize(&self) -> Result<Vec<u8>, Error>;
}

/// Serialization of UTF-8 based Strings to the quassel byteprotocol
pub trait SerializeUTF8 {
    fn serialize_utf8(&self) -> Result<Vec<u8>, Error>;
}

/// Deserialization of types and structs to the quassel byteprotocol
pub trait Deserialize {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error>
    where
        Self: std::marker::Sized;
}

/// Deserialization of UTF-8 based Strings to the quassel byteprotocol
pub trait DeserializeUTF8 {
    fn parse_utf8(b: &[u8]) -> Result<(usize, Self), Error>
    where
        Self: std::marker::Sized;
}

/// HandshakeSerialize implements the serialization needed during the handhake phase.
///
/// The protocol has some minor differences during this phase compared to the regular parsing.
pub trait HandshakeSerialize {
    fn serialize(&self) -> Result<Vec<u8>, Error>;
}

/// HandshakeDeserialize implements the deserialization needed during the handhake phase.
///
/// The protocol has some minor differences during this phase compared to the regular parsing.
pub trait HandshakeDeserialize {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error>
    where
        Self: std::marker::Sized;
}
