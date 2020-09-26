use crate::error::ProtocolError;
use crate::primitive::{Variant, VariantMap};
use crate::{HandshakeDeserialize, HandshakeSerialize};

use failure::Error;

/// ClientInitReject is received when the initialization fails
#[derive(Debug)]
pub struct ClientInitReject {
    /// String with an error message of what went wrong
    pub error_string: String,
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
            Variant::String(self.error_string.clone()),
        );
        return HandshakeSerialize::serialize(&values);
    }
}

impl HandshakeDeserialize for ClientInitReject {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (len, values): (usize, VariantMap) = HandshakeDeserialize::parse(b)?;

        let msgtype = match_variant!(&values["MsgType"], Variant::StringUTF8);

        if msgtype == "ClientInitReject" {
            return Ok((
                len,
                Self {
                    error_string: match_variant!(values["ErrorString"], Variant::String),
                },
            ));
        } else {
            bail!(ProtocolError::WrongMsgType);
        }
    }
}
