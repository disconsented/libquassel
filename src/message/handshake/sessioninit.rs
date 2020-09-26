use crate::error::ProtocolError;
use crate::primitive::{Variant, VariantList, VariantMap};
use crate::{HandshakeDeserialize, HandshakeSerialize};

use failure::Error;

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
