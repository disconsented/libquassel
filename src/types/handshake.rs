use crate::types::{Serialize, Deserialize};
use crate::types::variant::{Variant, VariantMap, VariantList};
use crate::types::basic::{String, StringList};

pub struct ClientInit {
    pub client_version: String, // Version of the client
    pub build_date: String, // Build date of the client
    pub client_features: u32, // Flags of supported legacy features
    pub feature_list: StringList // List of supported extended features
}

impl Serialize for ClientInit {
    fn serialize(&self) -> Vec<u8> {
        let mut values: VariantMap = VariantMap::with_capacity(5);
        values.insert("MsgType".to_string(), Variant::String("ClientInit".to_string()));
        values.insert("ClientVersion".to_string(), Variant::String(self.client_version.clone()));
        values.insert("BuildDate".to_string(), Variant::String(self.build_date.clone()));
        values.insert("ClientFeatures".to_string(), Variant::u32(self.client_features));
        values.insert("FeatureList".to_string(), Variant::StringList(self.feature_list.clone()));
        return values.serialize();
    }
}

// impl Deserialize for ClientInit {
//     fn deserialize(&mut self, b: &[u8]) {
//
//     }
// }

pub struct ClientInitReject {
    pub error_string: String
}

impl Serialize for ClientInitReject {
    fn serialize(&self) -> Vec<u8> {
        let mut values: VariantMap = VariantMap::with_capacity(2);
        values.insert("MsgTypes".to_string(), Variant::String("ClientInitReject".to_string()));
        values.insert("ErrorString".to_string(), Variant::String(self.error_string.clone()));
        return values.serialize();
    }
}

pub struct ClientInitAck {
    pub core_features: u32, // Flags of supported legacy features
    pub core_configured: bool, // If the core has already been configured
    pub backend_info: VariantList, // List of VariantMaps of info on available backends
    pub authenticator_info: VariantList, // List of VariantMaps of info on available authenticators
    pub feature_list: StringList, // List of supported extended features
}

impl Serialize for ClientInitAck {
    fn serialize(&self) -> Vec<u8> {
        let mut values: VariantMap = VariantMap::with_capacity(2);
        values.insert("MsgTypes".to_string(), Variant::String("ClientInitAck".to_string()));
        values.insert("CoreFeatures".to_string(), Variant::u32(self.core_features));
        values.insert("CoreConfigured".to_string(), Variant::bool(self.core_configured));
        values.insert("BackendInfo".to_string(), Variant::VariantList(self.backend_info.clone()));
        values.insert("AuthenticatorInfo".to_string(), Variant::VariantList(self.authenticator_info.clone()));
        values.insert("FeatureList".to_string(), Variant::StringList(self.feature_list.clone()));
        return values.serialize();
    }
}
