use crate::protocol::message::handshake::{VariantMap, HandshakeSerialize, HandshakeDeserialize};
use crate::protocol::primitive::{Variant};

#[test]
pub fn serialize_variantmap() {
    let mut test_variantmap = VariantMap::new();
    test_variantmap.insert("Configured".to_string(), Variant::bool(true));
    let bytes = [0, 0, 0, 43, 0, 0, 0, 10, 0, 0, 0, 10, 0,
         0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0, 100,
         0, 0, 0, 1, 0, 1].to_vec();
    assert_eq!(
        test_variantmap.serialize(),
        bytes
    );
}

// #[test]
// pub fn deserialize_variantmap() {
//     let test_bytes: &[u8] = &[0, 0, 0, 43, 0, 0, 0, 10, 0, 0, 0, 10, 0,
//          0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0, 100,
//          0, 0, 0, 1, 0, 1, 0, 0, 0, 1];
//     let mut test_variantmap = VariantMap::new();
//     test_variantmap.insert("Configured".to_string(), Variant::bool(true));
//
//     let (len, res) = VariantMap::parse(test_bytes);
//
//     assert_eq!(len, 43);
//     assert_eq!(res, test_variantmap);
// }
