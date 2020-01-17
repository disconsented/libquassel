use crate::protocol::message::handshake::{VariantMap, HandshakeSerialize, HandshakeDeserialize, HandshakeQRead};
use crate::protocol::primitive::{Variant};

#[test]
pub fn serialize_variantmap() {
    let mut test_variantmap = VariantMap::new();
    test_variantmap.insert("Configured".to_string(), Variant::bool(true));
    let bytes = [0, 0, 0, 39, 0, 0, 0, 10, 0, 0, 0, 10, 0,
         0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0, 100,
         0, 0, 0, 1, 0, 1].to_vec();
    assert_eq!(
        test_variantmap.serialize(),
        bytes
    );
}

#[test]
pub fn read_variantmap() {
    use std::io::Cursor;

    let test_bytes: Vec<u8> = vec![0, 0, 0, 39, 0, 0, 0, 10, 0, 0, 0, 10, 0,
        0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0, 100,
        0, 0, 0, 1, 0, 1, 0, 0, 0, 1];

    let mut buf: Vec<u8> = [0; 43].to_vec();
    let len = VariantMap::read(&mut Cursor::new(&test_bytes), &mut buf);

    assert_eq!(len, 43);

    let result_bytes: Vec<u8> = vec![0, 0, 0, 39, 0, 0, 0, 10, 0, 0, 0, 10, 0,
        0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0, 100,
        0, 0, 0, 1, 0, 1];
    assert_eq!(buf, result_bytes);
}

#[test]
pub fn deserialize_variantmap() {
    let test_bytes: &[u8] = &[0, 0, 0, 39, 0, 0, 0, 10, 0, 0, 0, 10, 0,
         0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0, 100,
         0, 0, 0, 1, 0, 1, 0, 0, 0, 1];
    let mut test_variantmap = VariantMap::new();
    test_variantmap.insert("Configured".to_string(), Variant::bool(true));

    let (len, res) = VariantMap::parse(test_bytes);

    assert_eq!(len, 43);
    assert_eq!(res, test_variantmap);
}

#[test]
pub fn deserialize_variantmap_utf8() {
    let test_bytes: &[u8] = &[0, 0, 0, 29, 0, 0, 0, 10, 0, 0, 0, 12, 0,
         0, 0, 0, 10, 67, 111, 110, 102, 105, 103, 117, 114, 101, 100,
         0, 0, 0, 1, 0, 1, 0, 0, 0, 1];
    let mut test_variantmap = VariantMap::new();
    test_variantmap.insert("Configured".to_string(), Variant::bool(true));

    let (len, res) = VariantMap::parse(test_bytes);

    assert_eq!(len, 33);
    assert_eq!(res, test_variantmap);
}
