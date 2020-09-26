use crate::Deserialize;
use crate::Serialize;

pub enum Protocol {
    Legacy = 0x00000001,
    Datastream = 0x00000002,
}

impl Protocol {
    pub fn new() -> Self {
        Protocol::Datastream
    }

    pub fn serialize(self) -> Vec<u8> {
        let proto: u32 = 0x80000002;

        proto.serialize().unwrap()
    }

    pub fn parse(buf: &[u8]) -> Self {
        let mut protolist: Vec<u32> = Vec::new();
        let mut pos = 0;
        loop {
            let (_, proto) = u32::parse(&buf[pos..(pos + 4)]).unwrap();
            if (proto & 0x80000000) >= 1 {
                protolist.push(proto - 0x80000000);
                break;
            } else {
                protolist.push(proto);
                pos += 4;
            }
        }

        Protocol::Datastream
    }
}
