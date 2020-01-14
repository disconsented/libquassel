use crate::protocol::primitive::serialize::{Serialize, SerializeUTF8};
use crate::protocol::primitive::deserialize::{Deserialize, DeserializeUTF8};

use crate::protocol::primitive::*;

#[test]
pub fn serialize_string() {
    let test_string: String = String::from("Configured");

    assert_eq!(test_string.serialize(), [0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0, 100]);
}

#[test]
pub fn serialize_string_utf8() {
    let test_string: String = String::from("Configured");

    assert_eq!(test_string.serialize_utf8(), [0, 0, 0, 10, 67, 111, 110, 102, 105, 103, 117, 114, 101, 100]);
}

#[test]
pub fn deserialize_string() {
    let test_bytes: &[u8] = &[0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0, 100, 0, 0, 0, 1];
    let (len, res) = String::parse(test_bytes);
    assert_eq!(res, "Configured");
    assert_eq!(len, 24);
}

#[test]
pub fn deserialize_string_utf8() {
    let test_bytes: &[u8] = &[0, 0, 0, 10, 67, 111, 110, 102, 105, 103, 117, 114, 101, 100, 0, 0, 0, 1];
    let (len, res) = String::parse_utf8(test_bytes);
    assert_eq!(len, 14);
    assert_eq!(res, "Configured");
}

#[test]
pub fn serialize_string_list() {
    let mut test_list = StringList::new();
    test_list.push("Configured".to_string());
    assert_eq!(
        test_list.serialize(),
        [0, 0, 0, 1, 0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0, 100]
    )
}

#[test]
pub fn deserialize_string_list() {
    let test_bytes: &[u8] = &[0, 0, 0, 24, 0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0, 100, 0, 0, 0, 1];
    let mut test_list = StringList::new();
    test_list.push("Configured".to_string());
    println!("aaaaa");
    let (len, res) = StringList::parse(test_bytes);
    assert_eq!(len, 28);
    assert_eq!(test_list, res);
}
