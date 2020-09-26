use crate::error::ProtocolError;
use crate::primitive::{StringList, Variant, VariantMap};
use crate::{HandshakeDeserialize, HandshakeSerialize};

use failure::Error;

/// ClientInit is the Initial message send to the core after establishing a base layer comunication.
///
/// Features
///
/// | Flag | Name | Description |
/// | ---- | ---- | ----------- |
/// | 0x00000001 | SynchronizedMarkerLine | -- |
/// | 0x00000002 | SaslAuthentication | -- |
/// | 0x00000004 | SaslExternal | -- |
/// | 0x00000008 | HideInactiveNetworks | -- |
/// | 0x00000010 | PasswordChange | -- |
/// | 0x00000020 | CapNegotiation | IRCv3 capability negotiation, account tracking |
/// | 0x00000040 | VerifyServerSSL | IRC server SSL validation |
/// | 0x00000080 | CustomRateLimits | IRC server custom message rate limits |
/// | 0x00000100 | DccFileTransfer | Currently not supported |
/// | 0x00000200 | AwayFormatTimestamp | Timestamp formatting in away (e.g. %%hh:mm%%) |
/// | 0x00000400 | Authenticators | Support for exchangeable auth backends |
/// | 0x00000800 | BufferActivitySync | Sync buffer activity status |
/// | 0x00001000 | CoreSideHighlights | Core-Side highlight configuration and matching |
/// | 0x00002000 | SenderPrefixes | Show prefixes for senders in backlog |
/// | 0x00004000 | RemoteDisconnect | Supports RPC call disconnectFromCore to remotely disconnect a client |
/// | 0x00008000 | ExtendedFeatures | Transmit features as list of strings |
/// | --         | LongTime | Serialize message time as 64-bit |
/// | --         | RichMessages | Real Name and Avatar URL in backlog |
/// | --         | BacklogFilterType | Backlogmanager supports filtering backlog by messagetype |
/// | --         | EcdsaCertfpKeys | ECDSA keys for CertFP in identities |
/// | --         | LongMessageId | 64-bit IDs for messages |
/// | --         | SyncedCoreInfo | CoreInfo dynamically updated using signals |
#[derive(Debug)]
pub struct ClientInit {
    /// Version of the client
    pub client_version: String,
    /// Build date of the client
    pub client_date: String,
    /// supported features as bitflags
    pub client_features: u32,
    /// List of supported extended features
    pub feature_list: StringList,
}

impl HandshakeSerialize for ClientInit {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut values: VariantMap = VariantMap::with_capacity(5);
        values.insert(
            "MsgType".to_string(),
            Variant::String("ClientInit".to_string()),
        );
        values.insert(
            "ClientVersion".to_string(),
            Variant::String(self.client_version.clone()),
        );
        values.insert(
            "ClientDate".to_string(),
            Variant::String(self.client_date.clone()),
        );
        values.insert("Features".to_string(), Variant::u32(self.client_features));
        values.insert(
            "FeatureList".to_string(),
            Variant::StringList(self.feature_list.clone()),
        );
        return HandshakeSerialize::serialize(&values);
    }
}

impl HandshakeDeserialize for ClientInit {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (len, values): (usize, VariantMap) = HandshakeDeserialize::parse(b)?;

        let msgtype = match_variant!(&values["MsgType"], Variant::StringUTF8);

        if msgtype == "ClientInit" {
            return Ok((
                len,
                Self {
                    client_version: match_variant!(values["ClientVersion"], Variant::String),
                    client_date: match_variant!(values["ClientDate"], Variant::String),
                    feature_list: match_variant!(values["FeatureList"], Variant::StringList),
                    client_features: match_variant!(values["Features"], Variant::u32),
                },
            ));
        } else {
            bail!(ProtocolError::WrongMsgType);
        }
    }
}
