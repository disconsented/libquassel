#[allow(dead_code)]
pub mod basic;
pub mod variant;

pub use basic::*;
pub use variant::*;

pub mod serialize {
    pub trait Serialize {
        fn serialize(&self) -> Vec<u8>;
    }
    pub trait SerializeUTF8 {
        fn serialize_utf8(&self) -> Vec<u8>;
    }
}

pub mod deserialize {
    pub trait Deserialize {
        fn parse(b: &[u8]) -> (usize, Self);
    }
    pub trait DeserializeUTF8 {
        fn parse_utf8(b: &[u8]) -> (usize, Self);
    }
}

pub mod qread {
    pub trait QRead {
        fn read<T: std::io::Read>(stream: &mut T, buf: &mut [u8]) -> usize;
    }
}
