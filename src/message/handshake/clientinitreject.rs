use crate::primitive::{Variant, VariantMap};
use crate::HandshakeSerialize;

use failure::Error;

/// ClientInitReject is received when the initialization fails
#[derive(Debug, Clone)]
pub struct ClientInitReject {
    /// String with an error message of what went wrong
    pub error: String,
}

impl HandshakeSerialize for ClientInitReject {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut values: VariantMap = VariantMap::with_capacity(2);
        values.insert(
            "MsgType".to_string(),
            Variant::String("ClientInitReject".to_string()),
        );
        values.insert(
            "ErrorString".to_string(),
            Variant::String(self.error.clone()),
        );
        return HandshakeSerialize::serialize(&values);
    }
}

impl From<VariantMap> for ClientInitReject {
    fn from(input: VariantMap) -> Self {
        ClientInitReject {
            error: match_variant!(input.get("ErrorString").unwrap(), Variant::String),
        }
    }
}
