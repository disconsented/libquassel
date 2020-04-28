use crate::Deserialize;
use crate::Serialize;

use crate::primitive::{BufferInfo, BufferType, Message, Variant, VariantList, VariantMap};

#[test]
pub fn serialize_variant_bool() {
    let test_variant_true = Variant::bool(true);
    let test_variant_false = Variant::bool(false);
    assert_eq!(test_variant_true.serialize().unwrap(), [0, 0, 0, 1, 0, 1]);
    assert_eq!(test_variant_false.serialize().unwrap(), [0, 0, 0, 1, 0, 0]);
}

#[test]
pub fn deserialize_variant_bool() {
    let test_bytes: &[u8] = &[0, 0, 0, 1, 0, 1, 0, 0, 0, 1];
    let (len, res) = Variant::parse(test_bytes).unwrap();
    assert_eq!(len, 6);
    assert_eq!(res, Variant::bool(true));
}

#[test]
pub fn serialize_variantlist() {
    let mut test_variantlist = VariantList::new();
    test_variantlist.push(Variant::bool(true));
    assert_eq!(
        test_variantlist.serialize().unwrap(),
        [0, 0, 0, 1, 0, 0, 0, 1, 0, 1]
    );
}

#[test]
pub fn deserialize_variantlist() {
    let test_bytes: &[u8] = &[0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1];
    let (len, res) = VariantList::parse(test_bytes).unwrap();
    let mut test_variantlist = VariantList::new();
    test_variantlist.push(Variant::bool(true));
    assert_eq!(len, 10);
    assert_eq!(res, test_variantlist);
}

#[test]
pub fn serialize_variantmap() {
    let mut test_variantmap = VariantMap::new();
    test_variantmap.insert("Configured".to_string(), Variant::bool(true));
    let bytes = [
        0, 0, 0, 1, 0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0,
        101, 0, 100, 0, 0, 0, 1, 0, 1,
    ]
    .to_vec();
    assert_eq!(test_variantmap.serialize().unwrap(), bytes);
}

#[test]
pub fn deserialize_variantmap() {
    let test_bytes: &[u8] = &[
        0, 0, 0, 1, 0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0,
        101, 0, 100, 0, 0, 0, 1, 0, 1,
    ];
    let (len, res) = VariantMap::parse(test_bytes).unwrap();
    let mut test_variantmap = VariantMap::new();
    test_variantmap.insert("Configured".to_string(), Variant::bool(true));
    assert_eq!(len, 34);
    assert_eq!(res, test_variantmap);
}

#[test]
pub fn serialize_buffer_info() {
    let test_buffer_info = BufferInfo {
        id: 0,
        network_id: 0,
        buffer_type: BufferType::Status,
        name: "test".to_string(),
    };

    let bytes = vec![
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5,
        0x74, 0x65, 0x73, 0x74, 0x0,
    ];
    assert_eq!(test_buffer_info.serialize().unwrap(), bytes);
}

#[test]
pub fn deserialize_buffer_info() {
    let test_buffer_info = BufferInfo {
        id: 0,
        network_id: 0,
        buffer_type: BufferType::Status,
        name: "test".to_string(),
    };

    let bytes = vec![
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x5,
        0x74, 0x65, 0x73, 0x74, 0x0,
    ];
    let (len, res) = BufferInfo::parse(&bytes).unwrap();

    assert_eq!(len, 23);
    assert_eq!(res, test_buffer_info);
}
