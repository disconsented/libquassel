use crate::protocol::message::handshake::{HandshakeDeserialize, HandshakeSerialize, VariantMap};
use crate::protocol::primitive::Variant;

#[test]
pub fn serialize_variantmap() {
    let mut test_variantmap = VariantMap::new();
    test_variantmap.insert("Configured".to_string(), Variant::bool(true));
    let bytes = [
        0, 0, 0, 2, 0, 0, 0, 10, 0, 0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0,
        117, 0, 114, 0, 101, 0, 100, 0, 0, 0, 1, 0, 1,
    ]
    .to_vec();
    assert_eq!(test_variantmap.serialize().unwrap(), bytes);
}

#[test]
pub fn deserialize_variantmap() {
    let test_bytes: &[u8] = &[
        0, 0, 0, 2, 0, 0, 0, 10, 0, 0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0,
        117, 0, 114, 0, 101, 0, 100, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1,
    ];
    let mut test_variantmap = VariantMap::new();
    test_variantmap.insert("Configured".to_string(), Variant::bool(true));

    let (len, res) = VariantMap::parse(test_bytes).unwrap();

    assert_eq!(len, 39);
    assert_eq!(res, test_variantmap);
}

#[test]
pub fn deserialize_variantmap_utf8() {
    let test_bytes: &[u8] = &[
        0, 0, 0, 2, 0, 0, 0, 12, 0, 0, 0, 0, 10, 67, 111, 110, 102, 105, 103, 117, 114, 101, 100,
        0, 0, 0, 1, 0, 1, 0, 0, 0, 1,
    ];
    let mut test_variantmap = VariantMap::new();
    test_variantmap.insert("Configured".to_string(), Variant::bool(true));

    let (len, res) = VariantMap::parse(test_bytes).unwrap();

    assert_eq!(len, 29);
    assert_eq!(res, test_variantmap);
}
