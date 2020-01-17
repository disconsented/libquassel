use std::result::Result;

use crate::protocol::error::ErrorKind;
use crate::protocol::primitive::{String, StringList, Variant, VariantList};

mod types;
pub use types::{VariantMap, HandshakeDeserialize, HandshakeSerialize, HandshakeQRead};

use crate::match_variant;
#[derive(Debug)]
pub struct ClientInit {
    pub client_version: String, // Version of the client
    pub client_date: String, // Build date of the client
    pub client_features: u32,
    pub feature_list: StringList // List of supported extended features
}

impl HandshakeSerialize for ClientInit {
    fn serialize(&self) -> Result<Vec<u8>, ErrorKind> {
        let mut values: VariantMap = VariantMap::with_capacity(5);
        values.insert("MsgType".to_string(), Variant::String("ClientInit".to_string()));
        values.insert("ClientVersion".to_string(), Variant::String(self.client_version.clone()));
        values.insert("ClientDate".to_string(), Variant::String(self.client_date.clone()));
        values.insert("Features".to_string(), Variant::u32(self.client_features));
        values.insert("FeatureList".to_string(), Variant::StringList(self.feature_list.clone()));
        return HandshakeSerialize::serialize(&values);
    }
}

impl HandshakeDeserialize for ClientInit {
    fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        let (len, values): (usize, VariantMap) = HandshakeDeserialize::parse(b)?;

        let msgtypev = &values["MsgType"];
        let msgtype;
        match msgtypev {
            Variant::String(x) => msgtype = x,
            Variant::StringUTF8(x) => msgtype = x,
            _ => return Err(ErrorKind::WrongVariant)
        };

        if msgtype == "ClientInit" {
            return Ok((len, Self {
                client_version: match_variant!(values, Variant::String, "ClientVersion"),
                client_date: match_variant!(values, Variant::String, "ClientDate"),
                feature_list: match_variant!(values, Variant::StringList, "FeatureList"),
                client_features: match_variant!(values, Variant::u32, "Features")
            }));
        } else {
            return Err(ErrorKind::WrongMsgType);
        }
    }
}

#[derive(Debug)]
pub struct ClientInitReject {
    pub error_string: String
}

impl HandshakeSerialize for ClientInitReject {
    fn serialize(&self) -> Result<Vec<u8>, ErrorKind> {
        let mut values: VariantMap = VariantMap::with_capacity(2);
        values.insert("MsgType".to_string(), Variant::String("ClientInitReject".to_string()));
        values.insert("ErrorString".to_string(), Variant::String(self.error_string.clone()));
        return HandshakeSerialize::serialize(&values);
    }
}

impl HandshakeDeserialize for ClientInitReject {
    fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        let (len, values): (usize, VariantMap) = HandshakeDeserialize::parse(b)?;

        let msgtypev = &values["MsgType"];
        let msgtype;
        match msgtypev {
            Variant::String(x) => msgtype = x,
            Variant::StringUTF8(x) => msgtype = x,
            _ => return Err(ErrorKind::WrongVariant)
        };

        if msgtype == "ClientInitReject" {
            return Ok((len, Self {
                error_string: match_variant!(values, Variant::String, "ErrorString")
            }));
        } else {
            return Err(ErrorKind::WrongMsgType);
        }
    }
}

#[derive(Debug)]
pub struct ClientInitAck {
    pub core_features: u32, // Flags of supported legacy features
    pub core_configured: bool, // If the core has already been configured
    pub storage_backends: VariantList, // List of VariantMaps of info on available backends
    pub authenticators: VariantList, // List of VariantMaps of info on available authenticators
    pub feature_list: StringList, // List of supported extended features
}

impl HandshakeSerialize for ClientInitAck {
    fn serialize(&self) -> Result<Vec<u8>, ErrorKind> {
        let mut values: VariantMap = VariantMap::with_capacity(2);
        values.insert("MsgType".to_string(), Variant::String("ClientInitAck".to_string()));
        values.insert("CoreFeatures".to_string(), Variant::u32(self.core_features));
        values.insert("Configured".to_string(), Variant::bool(self.core_configured));
        values.insert("StorageBackends".to_string(), Variant::VariantList(self.storage_backends.clone()));
        values.insert("Authenticators".to_string(), Variant::VariantList(self.authenticators.clone()));
        values.insert("FeatureList".to_string(), Variant::StringList(self.feature_list.clone()));
        return HandshakeSerialize::serialize(&values);
    }
}

impl HandshakeDeserialize for ClientInitAck {
    fn parse(b: &[u8]) -> Result<(usize, Self), ErrorKind> {
        let (len, values): (usize, VariantMap) = HandshakeDeserialize::parse(b)?;

        let msgtypev = &values["MsgType"];
        let msgtype;
        match msgtypev {
            Variant::String(x) => msgtype = x,
            Variant::StringUTF8(x) => msgtype = x,
            _ => return Err(ErrorKind::WrongVariant)
        };

        if msgtype == "ClientInitAck" {
            return Ok((len, Self {
                core_features: 0x00008000,
                core_configured: match_variant!(values, Variant::bool, "Configured"),
                storage_backends: match_variant!(values, Variant::VariantList, "StorageBackends"),
                authenticators: match_variant!(values, Variant::VariantList, "Authenticators"),
                feature_list: match_variant!(values, Variant::StringList, "FeatureList")
            }));
        } else {
            return Err(ErrorKind::WrongMsgType);
        }
    }
}
