use crate::Deserialize;
use crate::Serialize;

use crate::primitive::DateTime;

#[test]
pub fn serialize_datetime() {
    // Create datetime object
    let datetime = DateTime::parse("2020-02-19 13:00 +0200", "%Y-%m-%d %R %z").unwrap();
    println!("datetime: {:?}", datetime);

    let sers = datetime.serialize().unwrap();
    let bytes = vec![0, 37, 133, 19, 2, 202, 28, 128, 3, 0, 0, 28, 32];

    assert_eq!(sers, bytes)
}

#[test]
pub fn deserialize_datetime() {
    // Create datetime object
    let datetime = DateTime::parse("2020-02-19 13:00 +0200", "%Y-%m-%d %R %z").unwrap();
    println!("datetime: {:?}", datetime);

    let bytes = vec![0, 37, 133, 19, 2, 202, 28, 128, 3, 0, 0, 28, 32];
    let (_, res): (usize, DateTime) = Deserialize::parse(&bytes).unwrap();

    assert_eq!(res, datetime)
}
