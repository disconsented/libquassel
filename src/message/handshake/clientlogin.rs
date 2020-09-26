use crate::error::ProtocolError;
use crate::primitive::{Variant, VariantMap};
use crate::{HandshakeDeserialize, HandshakeSerialize};

use failure::Error;

/// Login to the core with user data
/// username and password are transmitted in plain text
#[derive(Debug)]
pub struct ClientLogin {
    pub user: String,
    pub password: String,
}

impl HandshakeSerialize for ClientLogin {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut values: VariantMap = VariantMap::new();
        values.insert(
            "MsgType".to_string(),
            Variant::String("ClientLogin".to_string()),
        );
        values.insert("User".to_string(), Variant::String(self.user.clone()));
        values.insert(
            "Password".to_string(),
            Variant::String(self.password.clone()),
        );
        return HandshakeSerialize::serialize(&values);
    }
}

impl HandshakeDeserialize for ClientLogin {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (len, values): (usize, VariantMap) = HandshakeDeserialize::parse(b)?;

        let msgtype = match_variant!(&values["MsgType"], Variant::StringUTF8);

        if msgtype == "ClientLogin" {
            return Ok((
                len,
                Self {
                    user: match_variant!(values["User"], Variant::String),
                    password: match_variant!(values["Password"], Variant::String),
                },
            ));
        } else {
            bail!(ProtocolError::WrongMsgType);
        }
    }
}
