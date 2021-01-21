use crate::primitive::{StringList, Variant, VariantList, VariantMap};

#[allow(unused_imports)]
use libquassel_derive::Network;

use std::collections::HashMap;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use super::{ircchannel::IrcChannel, ircuser::IrcUser, networkinfo::NetworkInfo};

#[derive(Debug, Clone)]
pub struct Network {
    my_nick: String,
    latency: i32,
    current_server: String,
    is_connected: bool,
    connection_state: ConnectionState,
    // prefixes: Vec<char>,
    // prefix_modes: Vec<char>,
    // channel_modes: HashMap<ChannelModeType, Vec<char>>,
    irc_users: HashMap<String, IrcUser>,
    irc_channels: HashMap<String, IrcChannel>,
    supports: HashMap<String, String>,
    caps: HashMap<String, String>,
    caps_enabled: Vec<String>,
    network_info: NetworkInfo,
}

// impl crate::message::signalproxy::Network for Network {
//     type Item = VariantList;

//     fn to_network(&self) -> Self::Item {
//         let mut res = Self::Item::new();

//         res.push(Variant::ByteArray(s!("myNick")));
//         res.push(Variant::String(self.my_nick.clone()));
//         res.push(Variant::ByteArray(s!("latency")));
//         res.push(Variant::i32(self.latency));
//         res.push(Variant::ByteArray(s!("currentServer")));
//         res.push(Variant::String(self.current_server.clone()));
//         res.push(Variant::ByteArray(s!("isConnected")));
//         res.push(Variant::bool(self.is_connected));
//         res.push(Variant::ByteArray(s!("connectionState")));
//         res.push(Variant::i32(self.connection_state.clone() as i32));

//         res.push(Variant::ByteArray(s!("Supports")));
//         res.push(Variant::VariantMap(
//             self.supports
//                 .iter()
//                 .map(|(k, v)| (k.clone(), Variant::String(v.clone())))
//                 .collect(),
//         ));

//         res.push(Variant::ByteArray(s!("Caps")));
//         res.push(Variant::VariantMap(
//             self.caps
//                 .iter()
//                 .map(|(k, v)| (k.clone(), Variant::String(v.clone())))
//                 .collect(),
//         ));

//         res.push(Variant::ByteArray(s!("CapsEnabled")));
//         res.push(Variant::VariantList(
//             self.caps_enabled
//                 .iter()
//                 .map(|v| Variant::String(v.clone()))
//                 .collect(),
//         ));

//         {
//             let mut map = VariantMap::new();

//             map.insert(
//                 s!("Users"),
//                 Variant::VariantMap(self.irc_users.iter().fold(
//                     HashMap::new(),
//                     |mut res, (_, v)| {
//                         res.extend(v.to_network());

//                         res
//                     },
//                 )),
//             );

//             let channels = self
//                 .irc_channels
//                 .iter()
//                 .fold(HashMap::new(), |mut res, (_, v)| {
//                     res.extend(v.to_network());

//                     res
//                 });

//             map.insert(s!("Channels"), Variant::VariantMap(channels));

//             res.push(Variant::ByteArray(s!("IrcUsersAndChannels")));
//             res.push(Variant::VariantMap(map));
//         }

//         res.extend(self.network_info.to_network());

//         res
//     }

//     fn from_network(input: Self::Item) -> Self {
//         let users_and_channels = match_variant!(
//             input
//                 .iter()
//                 .nth(
//                     input
//                         .iter()
//                         .position(|x| *x == Variant::ByteArray(s!("IrcUsersAndChannels")))
//                         .unwrap()
//                 )
//                 .unwrap(),
//             Variant::VariantMap
//         );

//         let res = Self {
//             my_nick: match_variant!(
//                 input
//                     .iter()
//                     .nth(
//                         input
//                             .iter()
//                             .position(|x| *x == Variant::ByteArray(s!("myNick")))
//                             .unwrap()
//                     )
//                     .unwrap(),
//                 Variant::String
//             ),
//             latency: match_variant!(
//                 input
//                     .iter()
//                     .nth(
//                         input
//                             .iter()
//                             .position(|x| *x == Variant::ByteArray(s!("latency")))
//                             .unwrap()
//                     )
//                     .unwrap(),
//                 Variant::i32
//             ),
//             current_server: match_variant!(
//                 input
//                     .iter()
//                     .nth(
//                         input
//                             .iter()
//                             .position(|x| *x == Variant::ByteArray(s!("currentServer")))
//                             .unwrap()
//                     )
//                     .unwrap(),
//                 Variant::String
//             ),
//             is_connected: match_variant!(
//                 input
//                     .iter()
//                     .nth(
//                         input
//                             .iter()
//                             .position(|x| *x == Variant::ByteArray(s!("isConnected")))
//                             .unwrap()
//                     )
//                     .unwrap(),
//                 Variant::bool
//             ),
//             connection_state: ConnectionState::from_i32(match_variant!(
//                 input
//                     .iter()
//                     .nth(
//                         input
//                             .iter()
//                             .position(|x| *x == Variant::ByteArray(s!("connectionState")))
//                             .unwrap()
//                     )
//                     .unwrap(),
//                 Variant::i32
//             ))
//             .unwrap(),
//             irc_users: match_variant!(
//                 users_and_channels.get("Users").unwrap(),
//                 Variant::VariantMap
//             )
//             .iter()
//             .map(|(k, v)| (k, IrcUser::from_network(v))),
//             irc_channels: match_variant!(
//                 users_and_channels.get("Channels").unwrap(),
//                 Variant::VariantMap
//             )
//             .iter()
//             .map(|(k, v)| (k, match_variant!(v, Variant::VariantList))),
//             supports: match_variant!(
//                 input
//                     .iter()
//                     .nth(
//                         input
//                             .iter()
//                             .position(|x| *x == Variant::ByteArray(s!("Supports")))
//                             .unwrap()
//                     )
//                     .unwrap(),
//                 Variant::VariantMap
//             )
//             .iter()
//             .map(|(k, v)| (k.clone(), match_variant!(v, Variant::String)))
//             .collect(),
//             caps: match_variant!(
//                 input
//                     .iter()
//                     .nth(
//                         input
//                             .iter()
//                             .position(|x| *x == Variant::ByteArray(s!("Caps")))
//                             .unwrap()
//                     )
//                     .unwrap(),
//                 Variant::VariantMap
//             )
//             .iter()
//             .map(|(k, v)| (k.clone(), match_variant!(v, Variant::String)))
//             .collect(),
//             caps_enabled: match_variant!(
//                 input
//                     .iter()
//                     .nth(
//                         input
//                             .iter()
//                             .position(|x| *x == Variant::ByteArray(s!("CapsEnabled")))
//                             .unwrap()
//                     )
//                     .unwrap(),
//                 Variant::VariantList
//             )
//             .iter()
//             .map(|v| match_variant!(v, Variant::String))
//             .collect(),
//             network_info: NetworkInfo::from_network(input),
//         };

//         todo!()
//     }
// }

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Network)]
#[network(repr = "map")]
pub struct NetworkServer {
    #[network(rename = "Host")]
    pub host: String,
    #[network(rename = "Port")]
    pub port: u32,
    #[network(rename = "Password")]
    pub password: String,
    #[network(rename = "UseSSL")]
    pub use_ssl: bool,
    #[network(rename = "sslVerify")]
    pub ssl_verify: bool,
    #[network(rename = "sslVersion")]
    pub ssl_version: i32,
    #[network(rename = "UseProxy")]
    pub use_proxy: bool,
    #[network(rename = "ProxyType")]
    pub proxy_type: i32,
    #[network(rename = "ProxyHost")]
    pub proxy_host: String,
    #[network(rename = "ProxyPort")]
    pub proxy_port: u32,
    #[network(rename = "ProxyUser")]
    pub proxy_user: String,
    #[network(rename = "ProxyPass")]
    pub proxy_pass: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::signalproxy::translation::Network;

    fn networkserver_get_network() -> VariantMap {
        map! {
            s!("ProxyHost") => Variant::String(
                s!("localhost"),
            ),
            s!("sslVerify") => Variant::bool(
                true,
            ),
            s!("UseSSL") => Variant::bool(
                true,
            ),
            s!("Port") => Variant::u32(
                6697,
            ),
            s!("Password") => Variant::String(
                s!(""),
            ),
            s!("ProxyType") => Variant::i32(
                1,
            ),
            s!("sslVersion") => Variant::i32(
                0,
            ),
            s!("ProxyUser") => Variant::String(
                s!(""),
            ),
            s!("ProxyPass") => Variant::String(
                s!(""),
            ),
            s!("Host") => Variant::String(
                s!("irc.snoonet.org"),
            ),
            s!("ProxyPort") => Variant::u32(
                8080,
            ),
            s!("UseProxy") => Variant::bool(
                false,
            ),
        }
    }
    fn networkserver_get_runtime() -> NetworkServer {
        NetworkServer {
            host: s!("irc.snoonet.org"),
            port: 6697,
            password: s!(""),
            use_ssl: true,
            ssl_verify: true,
            ssl_version: 0,
            use_proxy: false,
            proxy_type: 1,
            proxy_host: s!("localhost"),
            proxy_port: 8080,
            proxy_user: s!(""),
            proxy_pass: s!(""),
        }
    }

    #[test]
    fn network_server_to_network() {
        assert_eq!(
            networkserver_get_runtime().to_network(),
            networkserver_get_network()
        )
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
enum ConnectionState {
    Disconnected = 0x00,
    Connecting = 0x01,
    Initializing = 0x02,
    Initialized = 0x03,
    Reconnecting = 0x04,
    Disconnecting = 0x05,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
#[repr(C)]
enum ChannelModeType {
    NotAChanmode = 0x00,
    AChanmode = 0x01,
    BChanmode = 0x02,
    CChanmode = 0x04,
    DChanmode = 0x08,
}
