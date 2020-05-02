/// Match a VariantMaps field and return it's contents if successfull
///
/// # Example
///
/// ```
/// use libquassel::match_variant;
/// use libquassel::primitive::{VariantMap, Variant};
///
/// let var = Variant::String("test string".to_string());
/// let result = match_variant!(var, Variant::String);
/// ```
#[macro_export]
macro_rules! match_variant {
    ( $values:expr, $x:path ) => {
        match &$values {
            $x(x) => Ok(x.clone()),
            err => Err(err),
        }
        .unwrap();
    };
}

/// Prepend the length of `buf` to `buf`
pub fn prepend_byte_len(buf: &mut Vec<u8>) {
    use std::convert::TryInto;
    let len: i32 = buf.len().try_into().unwrap();
    let ulen: &[u8] = &len.to_be_bytes();
    buf.insert(0, ulen[3]);
    buf.insert(0, ulen[2]);
    buf.insert(0, ulen[1]);
    buf.insert(0, ulen[0]);
}

/// Insert a bytes `input` into `buf` at position `pos`
pub fn insert_bytes(pos: usize, buf: &mut Vec<u8>, input: &mut [u8]) {
    input.reverse();
    for i in input {
        buf.insert(pos, *i)
    }
}
