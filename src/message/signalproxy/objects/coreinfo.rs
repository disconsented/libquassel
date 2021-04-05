use libquassel_derive::Network;

use crate::primitive::{DateTime, StringList};

#[derive(Debug, Clone, PartialEq, Network)]
#[network(repr = "map")]
pub struct CoreInfo {
    #[network(rename = "coreData", network, variant = "VariantMap")]
    core_data: CoreData,
}

// // S->C calls
// setCoreData(coreData: QVariantMap)
// /**
//  * Replaces all properties of the object with the content of the
//  * "properties" parameter. This parameter is in network representation.
//  */
// update(properties: QVariantMap)

#[derive(Debug, Clone, PartialEq, Network)]
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
        network
    )]
    session_connected_client_data: Vec<ConnectedClient>,
}

#[derive(Debug, Clone, PartialEq, Network)]
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
