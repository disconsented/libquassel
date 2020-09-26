use crate::message::MessageType;
use crate::primitive::{Variant, VariantList};
use crate::{Deserialize, Serialize};

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
