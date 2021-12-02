use libquassel_derive::{NetworkList, NetworkMap};

use crate::message::signalproxy::translation::NetworkMap;
use crate::message::Syncable;
use crate::primitive::{DateTime, StringList};

#[derive(Debug, Clone, PartialEq, NetworkList, NetworkMap)]
#[network(repr = "map")]
pub struct CoreInfo {
    #[network(rename = "coreData", variant = "VariantMap", network)]
    core_data: CoreData,
}

impl CoreInfo {
    pub fn set_core_data(&mut self, data: CoreData) {
        #[cfg(feature = "server")]
        libquassel_derive::sync!("setCoreData", [data.to_network_map()]);

        self.core_data = data;
    }
}

#[cfg(feature = "client")]
impl crate::message::StatefulSyncableClient for CoreInfo {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "setCoreData" => self.set_core_data(CoreData::from_network_map(&mut get_param!(msg))),
            _ => (),
        }
    }

    /// Not Implemented
    fn request_update(&mut self)
    where
        Self: Sized,
    {
    }
}

#[cfg(feature = "server")]
impl crate::message::StatefulSyncableServer for CoreInfo {
    /// Not Implemented
    fn request_update(&mut self, mut _param: <CoreInfo as NetworkMap>::Item)
    where
        Self: Sized,
    {
    }
}

impl Syncable for CoreInfo {
    const CLASS: &'static str = "CoreInfo";
}

#[derive(Debug, Clone, PartialEq, NetworkMap)]
#[network(repr = "map")]
pub struct CoreData {
    #[network(rename = "quasselVersion")]
    quassel_version: String,
    #[network(rename = "quasselBuildDate")]
    quassel_build_date: String,
    #[network(rename = "startTime")]
    start_time: DateTime,
    #[network(rename = "sessionConnectedClients")]
    session_connected_clients: i32,
    #[network(
        rename = "sessionConnectedClientData",
        variant = "VariantList",
        network,
        map
    )]
    session_connected_client_data: Vec<ConnectedClient>,
}

#[derive(Debug, Clone, PartialEq, NetworkMap)]
#[network(repr = "map")]
pub struct ConnectedClient {
    #[network(rename = "id")]
    id: i32,
    #[network(rename = "remoteAddress")]
    remote_address: String,
    #[network(rename = "location")]
    location: String,
    #[network(rename = "clientVersion")]
    client_version: String,
    #[network(rename = "clientVersionDate")]
    client_version_date: String,
    #[network(rename = "connectedSince")]
    connected_since: DateTime,
    #[network(rename = "secure")]
    secure: bool,
    #[network(rename = "features")]
    features: i32,
    #[network(rename = "featureList")]
    feature_list: StringList,
}
