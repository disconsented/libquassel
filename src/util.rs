#[macro_export]
macro_rules! parse_match {
    ( $matchee:expr, $pos:expr, $map:expr, $bytes:expr, $name:expr, $(($pattern:pat, $type:ty, $variant:expr)),* ) => {
        match $matchee {
            $(
                $pattern => {
                    let value: $type;

                    $pos = $pos + value.parse(&$bytes[($pos)..]);
                    $map.insert($name, $variant(value));
                },
            )*
        };
    };
}

#[macro_export]
macro_rules! match_variant {
    ( $values:expr, $x:path, $field:expr ) => {
        match &$values[$field] {
            $x(x) => { Ok(x.clone()) },
            _ => { Err("") }
        }.unwrap();
    }
}

pub fn prepend_byte_len(buf: &mut Vec<u8>) {
    use std::convert::TryInto;
    let len: i32 = buf.len().try_into().unwrap();
    let ulen: &[u8] = &len.to_be_bytes();
    buf.insert(0, ulen[3]);
    buf.insert(0, ulen[2]);
    buf.insert(0, ulen[1]);
    buf.insert(0, ulen[0]);
}

pub fn insert_bytes(pos: usize, buf: &mut Vec<u8>, input: &mut [u8]) {
    input.reverse();
    for i in input {
        buf.insert(pos, *i)
    }
}
