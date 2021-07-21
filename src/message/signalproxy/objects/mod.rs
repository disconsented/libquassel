mod aliasmanager;
mod buffersyncer;
mod coreinfo;
mod highlightrulemanager;
mod identity;
mod ircchannel;
mod ircuser;
mod network;
mod networkinfo;

use std::convert::TryInto;

pub use aliasmanager::*;
pub use buffersyncer::*;
pub use coreinfo::*;
pub use highlightrulemanager::*;
pub use identity::*;
pub use ircchannel::*;
pub use ircuser::*;
pub use network::*;
pub use networkinfo::*;

use libquassel_derive::From;

use super::Network;
use crate::primitive::VariantList;

#[derive(Debug, Clone, PartialEq, From)]
pub enum Types {
    AliasManager(AliasManager),
    BufferSyncer(BufferSyncer),
    Network(network::Network),
    NetworkInfo(NetworkInfo),
    NetworkConfig(NetworkConfig),
    CoreData(CoreData),
    Unknown(VariantList),
}

impl Types {
    pub fn to_network(&self) -> VariantList {
        debug!("converting to network object: {:#?}", self);
        match self {
            Types::AliasManager(val) => val.to_network(),
            Types::BufferSyncer(val) => val.to_network(),
            Types::Network(val) => val.to_network(),
            Types::NetworkInfo(val) => val.to_network(),
            Types::NetworkConfig(val) => val.to_network(),
            Types::CoreData(val) => vec![val.to_network().into()],
            Types::Unknown(val) => val.clone(),
        }
    }

    pub fn from_network(class_name: &str, input: &mut VariantList) -> Self {
        debug!(
            "converting {} from network object: {:#?}",
            class_name, input
        );
        match class_name {
            "Network" => Types::Network(Network::from_network(input)),
            "NetworkInfo" => Types::NetworkInfo(NetworkInfo::from_network(input)),
            "NetworkConfig" => Types::NetworkConfig(NetworkConfig::from_network(input)),
            "AliasManager" => Types::AliasManager(AliasManager::from_network(input)),
            "BufferSyncer" => Types::BufferSyncer(BufferSyncer::from_network(input)),
            "CoreData" => Types::CoreData(CoreData::from_network(
                &mut input.remove(0).try_into().unwrap(),
            )),
            _ => Types::Unknown(input.to_owned()),
        }
    }
}
