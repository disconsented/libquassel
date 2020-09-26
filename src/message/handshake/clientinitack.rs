use crate::error::ProtocolError;
use crate::primitive::{StringList, Variant, VariantList, VariantMap};
use crate::{HandshakeDeserialize, HandshakeSerialize};

use failure::Error;

/// ClientInitAck is received when the initialization was successfull
#[derive(Debug)]
pub struct ClientInitAck {
    /// Flags of supported legacy features
    pub core_features: u32,
    /// If the core has already been configured
    pub core_configured: bool,
    /// List of VariantMaps of info on available backends
    pub storage_backends: VariantList,
    /// List of VariantMaps of info on available authenticators
    pub authenticators: VariantList,
    /// List of supported extended features
    pub feature_list: StringList,
}

impl HandshakeSerialize for ClientInitAck {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut values: VariantMap = VariantMap::with_capacity(6);
        values.insert(
            "MsgType".to_string(),
            Variant::String("ClientInitAck".to_string()),
        );
        values.insert("CoreFeatures".to_string(), Variant::u32(self.core_features));
        values.insert(
            "Configured".to_string(),
            Variant::bool(self.core_configured),
        );
        values.insert(
            "StorageBackends".to_string(),
            Variant::VariantList(self.storage_backends.clone()),
        );
        values.insert(
            "Authenticators".to_string(),
            Variant::VariantList(self.authenticators.clone()),
        );
        values.insert(
            "FeatureList".to_string(),
            Variant::StringList(self.feature_list.clone()),
        );
        return HandshakeSerialize::serialize(&values);
    }
}

impl HandshakeDeserialize for ClientInitAck {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (len, values): (usize, VariantMap) = HandshakeDeserialize::parse(b)?;

        let msgtype = match_variant!(&values["MsgType"], Variant::StringUTF8);

        if msgtype == "ClientInitAck" {
            return Ok((
                len,
                Self {
                    core_features: 0x00008000,
                    core_configured: match_variant!(values["Configured"], Variant::bool),
                    storage_backends: match_variant!(
                        values["StorageBackends"],
                        Variant::VariantList
                    ),
                    authenticators: match_variant!(values["Authenticators"], Variant::VariantList),
                    feature_list: match_variant!(values["FeatureList"], Variant::StringList),
                },
            ));
        } else {
            bail!(ProtocolError::WrongMsgType);
        }
    }
}
