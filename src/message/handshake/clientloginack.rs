use crate::error::ProtocolError;
use crate::primitive::{Variant, VariantMap};
use crate::{HandshakeDeserialize, HandshakeSerialize};

use failure::Error;

/// ClientLoginAck is received after the client has successfully logged in
/// it has no fields
#[derive(Debug, Clone)]
pub struct ClientLoginAck;

impl HandshakeSerialize for ClientLoginAck {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut values: VariantMap = VariantMap::with_capacity(1);
        values.insert(
            "MsgType".to_string(),
            Variant::String("ClientLoginAck".to_string()),
        );
        return HandshakeSerialize::serialize(&values);
    }
}

impl HandshakeDeserialize for ClientLoginAck {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (len, values): (usize, VariantMap) = HandshakeDeserialize::parse(b)?;

        let msgtype = match_variant!(&values["MsgType"], Variant::StringUTF8);

        if msgtype == "ClientLogin" {
            return Ok((len, Self {}));
        } else {
            bail!(ProtocolError::WrongMsgType);
        }
    }
}
