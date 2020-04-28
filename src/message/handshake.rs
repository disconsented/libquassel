use failure::Error;
use std::result::Result;

use crate::error::ProtocolError;
use crate::primitive::{StringList, Variant, VariantList};
mod types;
use crate::primitive::VariantMap;
use crate::{HandshakeDeserialize, HandshakeSerialize};

use crate::match_variant;

/// Data received right after initializing the connection
///
/// ConnAck is serialized sequentially
#[derive(Debug)]
pub struct ConnAck {
    /// The Flag 0x01 for TLS
    /// and 0x02 for Deflate Compression
    flags: u8,
    /// Some extra protocol version specific data
    /// So far unused
    extra: i16,
    /// The version of the protocol
    /// 0x00000001 for the legacy protocol
    /// 0x00000002 for the datastream protocol
    ///
    /// Only the datastream protocol is supported by this crate
    version: i8,
}

impl Default for ConnAck {
    fn default() -> Self {
        Self {
            flags: 0x00,
            extra: 0x00,
            version: 0x00000002,
        }
    }
}

impl crate::Serialize for ConnAck {
    fn serialize(&self) -> Result<Vec<std::primitive::u8>, Error> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.append(&mut self.flags.serialize()?);
        bytes.append(&mut self.extra.serialize()?);
        bytes.append(&mut self.version.serialize()?);

        Ok(bytes)
    }
}

impl crate::Deserialize for ConnAck {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (flen, flags) = u8::parse(b)?;
        let (elen, extra) = i16::parse(&b[flen..])?;
        let (vlen, version) = i8::parse(&b[(flen + elen)..])?;

        return Ok((
            flen + elen + vlen,
            Self {
                flags,
                extra,
                version,
            },
        ));
    }
}

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

/// ClientLoginAck is received after the client has successfully logged in
/// it has no fields
#[derive(Debug)]
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

/// SessionInit is received along with ClientLoginAck to initialize that user Session
// TODO Replace with proper types
#[derive(Debug)]
pub struct SessionInit {
    /// List of all configured identities
    identities: VariantList,
    /// List of all existing buffers
    buffers: VariantList,
    /// Ids of all networks
    network_ids: VariantList,
}

impl HandshakeSerialize for SessionInit {
    fn serialize(&self) -> Result<Vec<u8>, Error> {
        let mut values: VariantMap = VariantMap::with_capacity(1);
        values.insert(
            "MsgType".to_string(),
            Variant::String("SessionInit".to_string()),
        );
        values.insert(
            "Identities".to_string(),
            Variant::VariantList(self.identities.clone()),
        );
        values.insert(
            "BufferInfos".to_string(),
            Variant::VariantList(self.buffers.clone()),
        );
        values.insert(
            "NetworkIds".to_string(),
            Variant::VariantList(self.network_ids.clone()),
        );
        return HandshakeSerialize::serialize(&values);
    }
}

impl HandshakeDeserialize for SessionInit {
    fn parse(b: &[u8]) -> Result<(usize, Self), Error> {
        let (len, values): (usize, VariantMap) = HandshakeDeserialize::parse(b)?;

        let msgtype = match_variant!(&values["MsgType"], Variant::StringUTF8);

        if msgtype == "ClientLogin" {
            return Ok((
                len,
                Self {
                    identities: match_variant!(values["Identities"], Variant::VariantList),
                    buffers: match_variant!(values["BufferInfos"], Variant::VariantList),
                    network_ids: match_variant!(values["NetworkIds"], Variant::VariantList),
                },
            ));
        } else {
            bail!(ProtocolError::WrongMsgType);
        }
    }
}
