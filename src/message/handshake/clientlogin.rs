use crate::primitive::{Variant, VariantMap};
use crate::HandshakeSerialize;

use failure::Error;

/// Login to the core with user data
/// username and password are transmitted in plain text
#[derive(Debug, Clone)]
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

impl From<VariantMap> for ClientLogin {
    fn from(input: VariantMap) -> Self {
        ClientLogin {
            user: match_variant!(input.get("User").unwrap(), Variant::String),
            password: match_variant!(input.get("Password").unwrap(), Variant::String),
        }
    }
}
