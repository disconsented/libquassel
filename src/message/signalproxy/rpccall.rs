use crate::message::MessageType;
use crate::primitive::{Variant, VariantList};
use crate::{Deserialize, Serialize};

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
