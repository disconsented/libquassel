#[allow(dead_code)]
pub mod basic;
pub mod variant;

pub use basic::*;
pub use variant::*;


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
        fn parse(b: &[u8]) -> Result<(usize, Self), Error> where Self: std::marker::Sized ;
    }
    pub trait DeserializeUTF8 {
        fn parse_utf8(b: &[u8]) -> Result<(usize, Self), Error> where Self: std::marker::Sized ;
    }
}

pub mod qread {
    use failure::Error;
    pub trait QRead {
        fn read<T: std::io::Read>(stream: &mut T, buf: &mut [u8]) -> Result<usize, Error>;
    }
}
