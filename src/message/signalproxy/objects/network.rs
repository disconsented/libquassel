use std::convert::TryInto;

use crate::primitive::{Variant, VariantList, VariantMap};

use libquassel_derive::Network;

use std::collections::HashMap;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

use super::{ircchannel::IrcChannel, ircuser::IrcUser, networkinfo::NetworkInfo};

#[derive(Debug, Clone, PartialEq)]
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

impl crate::message::signalproxy::Network for Network {
    type Item = VariantList;

    fn to_network(&self) -> Self::Item {
        let mut res = Self::Item::new();

        res.push(Variant::ByteArray(s!("myNick")));
        res.push(Variant::String(self.my_nick.clone()));
        res.push(Variant::ByteArray(s!("latency")));
        res.push(Variant::i32(self.latency));
        res.push(Variant::ByteArray(s!("currentServer")));
        res.push(Variant::String(self.current_server.clone()));
        res.push(Variant::ByteArray(s!("isConnected")));
        res.push(Variant::bool(self.is_connected));
        res.push(Variant::ByteArray(s!("connectionState")));
        res.push(Variant::i32(self.connection_state.clone() as i32));

        res.push(Variant::ByteArray(s!("Supports")));
        res.push(Variant::VariantMap(
            self.supports
                .iter()
                .map(|(k, v)| (k.clone(), Variant::String(v.clone())))
                .collect(),
        ));

        res.push(Variant::ByteArray(s!("Caps")));
        res.push(Variant::VariantMap(
            self.caps
                .iter()
                .map(|(k, v)| (k.clone(), Variant::String(v.clone())))
                .collect(),
        ));

        res.push(Variant::ByteArray(s!("CapsEnabled")));
        res.push(Variant::VariantList(
            self.caps_enabled
                .iter()
                .map(|v| Variant::String(v.clone()))
                .collect(),
        ));

        {
            let mut map = VariantMap::new();

            map.insert(
                s!("Users"),
                Variant::VariantMap(self.irc_users.iter().fold(
                    HashMap::new(),
                    |mut res, (_, v)| {
                        res.extend(v.to_network());

                        res
                    },
                )),
            );

            let channels = self
                .irc_channels
                .iter()
                .fold(HashMap::new(), |mut res, (_, v)| {
                    res.extend(v.to_network());

                    res
                });

            map.insert(s!("Channels"), Variant::VariantMap(channels));

            res.push(Variant::ByteArray(s!("IrcUsersAndChannels")));
            res.push(Variant::VariantMap(map));
        }

        res.extend(self.network_info.to_network());

        res
    }

    fn from_network(input: &mut Self::Item) -> Self {
        let mut i = input.iter().cycle();

        let users_and_channels: VariantMap = {
            i.position(|x| *x == Variant::ByteArray(String::from("IrcUsersAndChannels")))
                .unwrap();

            i.next().unwrap().try_into().unwrap()
        };

        log::trace!("users and channels: {:#?}", users_and_channels);

        Self {
            my_nick: {
                i.position(|x| *x == Variant::ByteArray(String::from("myNick")))
                    .unwrap();

                i.next().unwrap().try_into().unwrap()
            },
            latency: {
                i.position(|x| *x == Variant::ByteArray(String::from("latency")))
                    .unwrap();

                i.next().unwrap().try_into().unwrap()
            },
            current_server: {
                i.position(|x| *x == Variant::ByteArray(String::from("currentServer")))
                    .unwrap();

                i.next().unwrap().try_into().unwrap()
            },
            is_connected: {
                i.position(|x| *x == Variant::ByteArray(String::from("isConnected")))
                    .unwrap();

                i.next().unwrap().try_into().unwrap()
            },
            connection_state: ConnectionState::from_i32({
                i.position(|x| *x == Variant::ByteArray(String::from("connectionState")))
                    .unwrap();

                i.next().unwrap().try_into().unwrap()
            })
            .unwrap(),
            irc_users: {
                let users: Vec<IrcUser> = Vec::<IrcUser>::from_network(
                    &mut users_and_channels.get("Users").unwrap().try_into().unwrap(),
                );
                users
                    .into_iter()
                    .map(|user| (user.nick.clone(), user))
                    .collect()
            },
            irc_channels: {
                let channels: Vec<IrcChannel> = Vec::<IrcChannel>::from_network(
                    &mut users_and_channels
                        .get("Channels")
                        .unwrap()
                        .try_into()
                        .unwrap(),
                );
                channels
                    .into_iter()
                    .map(|channel| (channel.name.clone(), channel))
                    .collect()
            },
            supports: {
                i.position(|x| *x == Variant::ByteArray(String::from("Supports")))
                    .unwrap();

                let var: VariantMap = i.next().unwrap().try_into().unwrap();

                var.into_iter()
                    .map(|(k, v)| (k, v.try_into().unwrap()))
                    .collect()
            },
            caps: {
                i.position(|x| *x == Variant::ByteArray(String::from("Caps")))
                    .unwrap();

                let var: VariantMap = i.next().unwrap().try_into().unwrap();

                var.into_iter()
                    .map(|(k, v)| (k, v.try_into().unwrap()))
                    .collect()
            },
            caps_enabled: {
                i.position(|x| *x == Variant::ByteArray(String::from("CapsEnabled")))
                    .unwrap();

                let var: VariantList = i.next().unwrap().try_into().unwrap();

                var.into_iter().map(|v| v.try_into().unwrap()).collect()
            },
            network_info: NetworkInfo::from_network(input),
        }
    }
}

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

#[derive(Debug, Clone, PartialEq, Network)]
#[network(repr = "list")]
pub struct NetworkConfig {
    #[network(rename = "pingTimeoutEnabled")]
    ping_timeout_enabled: bool,
    #[network(rename = "pingInterval")]
    ping_interval: i32,
    #[network(rename = "maxPingCount")]
    max_ping_count: i32,
    #[network(rename = "autoWhoEnabled")]
    auto_who_enabled: bool,
    #[network(rename = "autoWhoInterval")]
    auto_who_interval: i32,
    #[network(rename = "autoWhoNickLimit")]
    auto_who_nick_limit: i32,
    #[network(rename = "autoWhoDelay")]
    auto_who_delay: i32,
    #[network(rename = "standardCtcp")]
    standard_ctcp: bool,
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
