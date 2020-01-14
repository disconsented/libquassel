use crate::protocol::primitive::serialize::Serialize;
use crate::protocol::primitive::deserialize::Deserialize;

use crate::protocol::primitive::{Variant, VariantList, VariantMap};

#[test]
pub fn serialize_variant_bool() {
    let test_variant_true = Variant::bool(true);
    let test_variant_false = Variant::bool(false);
    assert_eq!(
        test_variant_true.serialize(),
        [0, 0, 0, 1, 0, 1]
    );
    assert_eq!(
        test_variant_false.serialize(),
        [0, 0, 0, 1, 0, 0]
    );
}

#[test]
pub fn deserialize_variant_bool() {
    let test_bytes: &[u8] = &[0, 0, 0, 1, 0, 1, 0, 0, 0, 1];
    let (len, res) = Variant::parse(test_bytes);
    assert_eq!(len, 6);
    assert_eq!(res, Variant::bool(true));
}

#[test]
pub fn serialize_variantlist() {
    let mut test_variantlist = VariantList::new();
    test_variantlist.push(Variant::bool(true));
    assert_eq!(
        test_variantlist.serialize(),
        [0, 0, 0, 1, 0, 0, 0, 1, 0, 1]
    );
}

#[test]
pub fn deserialize_variantlist() {
    let test_bytes: &[u8] = &[0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1];
    let (len, res) = VariantList::parse(test_bytes);
    let mut test_variantlist = VariantList::new();
    test_variantlist.push(Variant::bool(true));
    assert_eq!(len, 10);
    assert_eq!(res, test_variantlist);
}

#[test]
pub fn serialize_variantmap() {
    let mut test_variantmap = VariantMap::new();
    test_variantmap.insert("Configured".to_string(), Variant::bool(true));
    let bytes = [0, 0, 0, 1,
         0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0, 100,
         0, 0, 0, 1, 0, 1].to_vec();
    assert_eq!(
        test_variantmap.serialize(),
        bytes
    );
}

#[test]
pub fn deserialize_variantmap() {
    let test_bytes: &[u8] = &[0, 0, 0, 1,
         0, 0, 0, 20, 0, 67, 0, 111, 0, 110, 0, 102, 0, 105, 0, 103, 0, 117, 0, 114, 0, 101, 0, 100,
         0, 0, 0, 1, 0, 1, 0, 0, 0, 1];
    let (len, res) = VariantMap::parse(test_bytes);
    let mut test_variantmap = VariantMap::new();
    test_variantmap.insert("Configured".to_string(), Variant::bool(true));
    assert_eq!(len, 34);
    assert_eq!(res, test_variantmap);
}

//
// 0000   00 00 00 00 00 00 00 00 00 00 00 00 08 00 45 00   ..............E.
// 0010   00 e2 36 3d 40 00 40 06 05 d7 7f 00 00 01 7f 00   ..6=@.@.........
// 0020   00 01 10 92 c0 dc e8 73 50 fe 2f 68 6d 9e 80 18   .......sP./hm...
// 0030   02 00 fe d6 00 00 01 01 08 0a 5f 0a 31 00 5f 0a   .........._.1._.
// 0040   31 00 00 00 00 aa 00 00 00 0a 00 00 00 0c 00 00   1...............
// 0050   00 00 0a 43 6f 6e 66 69 67 75 72 65 64 00 00 00   ...Configured...
// 0060   01 00 01 00 00 00 0c 00 00 00 00 0c 43 6f 72 65   ............Core
// 0070   46 65 61 74 75 72 65 73 00 00 00 03 00 00 00 fe   Features........
// 0080   ff 00 00 00 0c 00 00 00 00 0c 4c 6f 67 69 6e 45   ..........LoginE
// 0090   6e 61 62 6c 65 64 00 00 00 01 00 01 00 00 00 0c   nabled..........
// 00a0   00 00 00 00 07 4d 73 67 54 79 70 65 00 00 00 0a   .....MsgType....
// 00b0   00 00 00 00 1a 00 43 00 6c 00 69 00 65 00 6e 00   ......C.l.i.e.n.
// 00c0   74 00 49 00 6e 00 69 00 74 00 41 00 63 00 6b 00   t.I.n.i.t.A.c.k.
// 00d0   00 00 0c 00 00 00 00 0f 53 74 6f 72 61 67 65 42   ........StorageB
// 00e0   61 63 6b 65 6e 64 73 00 00 00 09 00 00 00 00 00   ackends.........
