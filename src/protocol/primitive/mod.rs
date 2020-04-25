pub mod bufferinfo;
pub mod datetime;
pub mod message;
pub mod signedint;
pub mod string;
pub mod stringlist;
pub mod unsignedint;
pub mod variant;
pub mod variantlist;
pub mod variantmap;

pub use bufferinfo::*;
pub use datetime::*;
pub use message::*;
pub use signedint::*;
pub use string::*;
pub use stringlist::*;
pub use unsignedint::*;
pub use variant::*;
pub use variantlist::*;
pub use variantmap::*;

// Static Type Definitions
pub const VOID: u32 = 0x00000000;
pub const BOOL: u32 = 0x00000001;
pub const QCHAR: u32 = 0x00000007;

pub const QVARIANT: u32 = 0x00000090;
pub const QVARIANTMAP: u32 = 0x00000008;
pub const QVARIANTLIST: u32 = 0x00000009;

pub const QSTRING: u32 = 0x0000000a;
pub const QSTRINGLIST: u32 = 0x0000000b;
pub const QBYTEARRAY: u32 = 0x0000000c;

pub const QDATE: u32 = 0x0000000e;
pub const QTIME: u32 = 0x0000000f;
pub const QDATETIME: u32 = 0x00000010;
pub const USERTYPE: u32 = 0x0000007f;

// Basic types
pub const LONG: u32 = 0x00000081; // int64_t
pub const INT: u32 = 0x00000002; // int32_t
pub const SHORT: u32 = 0x00000082; // int16_t
pub const CHAR: u32 = 0x00000083; // int8_t

pub const ULONG: u32 = 0x00000084; // uint64_t
pub const UINT: u32 = 0x00000003; // uint32_t
pub const USHORT: u32 = 0x00000085; // uint16_t
pub const UCHAR: u32 = 0x00000086; // uint8_t

pub mod serialize {
    use failure::Error;
    pub trait Serialize {
        fn serialize(&self) -> Result<Vec<u8>, Error>;
    }
    pub trait SerializeUTF8 {
        fn serialize_utf8(&self) -> Result<Vec<u8>, Error>;
    }
}

pub mod deserialize {
    use failure::Error;
    pub trait Deserialize {
        fn parse(b: &[u8]) -> Result<(usize, Self), Error>
        where
            Self: std::marker::Sized;
    }
    pub trait DeserializeUTF8 {
        fn parse_utf8(b: &[u8]) -> Result<(usize, Self), Error>
        where
            Self: std::marker::Sized;
    }
}
