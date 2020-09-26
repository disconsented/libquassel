use crate::error::ProtocolError;
use crate::primitive::{Variant, VariantMap};
use crate::{HandshakeDeserialize, HandshakeSerialize};

use failure::Error;

/// ClientLoginReject is received after the client failed to login
/// It contains an error message as String
#[derive(Debug)]
pub struct ClientLoginReject {
    error: String,
}

impl HandshakeSerialize for ClientLoginReject {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut values: VariantMap = VariantMap::with_capacity(1);
        values.insert(
            "MsgType".to_string(),
            Variant::String("ClientLoginReject".to_string()),
        );
        values.insert(
            "ErrorString".to_string(),
            Variant::String(self.error.clone()),
        );
        return HandshakeSerialize::serialize(&values);
    }
}

impl HandshakeDeserialize for ClientLoginReject {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (len, values): (usize, VariantMap) = HandshakeDeserialize::parse(b)?;

        let msgtype = match_variant!(&values["MsgType"], Variant::StringUTF8);

        if msgtype == "ClientLogin" {
            return Ok((
                len,
                Self {
                    error: match_variant!(values["ErrorString"], Variant::String),
                },
            ));
        } else {
            bail!(ProtocolError::WrongMsgType);
        }
    }
}
