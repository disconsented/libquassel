#[allow(dead_code)]
pub mod basic;
pub mod variant;

pub use basic::*;
pub use variant::*;


pub mod serialize {
    use crate::protocol::error::ErrorKind;

    pub trait Serialize {
        fn serialize(&self) -> Result<Vec<u8>, ErrorKind>;
    }
    pub trait SerializeUTF8 {
        fn serialize_utf8(&self) -> Result<Vec<u8>, ErrorKind>;
    }
}

pub mod deserialize {
    use crate::protocol::error::ErrorKind;

    pub trait Deserialize {
        fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> where Self: std::marker::Sized ;
    }
    pub trait DeserializeUTF8 {
        fn parse_utf8(b: &[u8]) -> Result<(usize, Self), ErrorKind> where Self: std::marker::Sized ;
    }
}

pub mod qread {
    use crate::protocol::error::ErrorKind;

    pub trait QRead {
        fn read<T: std::io::Read>(stream: &mut T, buf: &mut [u8]) -> Result<usize, ErrorKind>;
    }
}
