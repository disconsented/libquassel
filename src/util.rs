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
