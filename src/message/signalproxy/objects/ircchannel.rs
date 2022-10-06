use std::collections::HashMap;

#[cfg(feature = "server")]
use libquassel_derive::sync;
use libquassel_derive::Setters;
use log::{error, warn};

use crate::message::{NetworkMap, Syncable, Class};
use crate::primitive::{StringList, Variant, VariantList, VariantMap};

use super::ChannelModeType;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Setters)]
pub struct IrcChannel {
    /// Modes that add or remove items from a list, like commonly +b for the banlist.
    ///
    /// Always require a parameter from server to client.
    /// Clients can request the whole list by leaving the parameter empty
    #[setter(skip)]
    pub channel_modes_a: HashMap<char, StringList>,

    /// Modes that take a parameter as setting and require it when setting or removing the mode.
    #[setter(skip)]
    pub channel_modes_b: HashMap<char, String>,

    /// Modes that take a parameter as setting, but only require it when setting the mode.
    #[setter(skip)]
    pub channel_modes_c: HashMap<char, String>,

    /// Modes without a parameter.
    #[setter(skip)]
    pub channel_modes_d: String,

    // pub channel_modes: HashMap<char, ChannelMode>,
    #[setter(skip)]
    pub user_modes: HashMap<String, String>,
    #[setter(skip)]
    pub name: String,

    pub topic: String,
    pub password: String,
    pub encrypted: bool,
}

// TODO keep user modes sorted
impl IrcChannel {
    pub fn add_channel_mode(&mut self, mode_type: ChannelModeType, mode: char, value: String) {
        match mode_type {
            ChannelModeType::NotAChanmode => (),
            ChannelModeType::AChanmode => {
                self.channel_modes_a.insert(mode, vec![value]);
            },
            ChannelModeType::BChanmode => {
                self.channel_modes_b.insert(mode, value);
            },
            ChannelModeType::CChanmode => {
                self.channel_modes_c.insert(mode, value);
            },
            ChannelModeType::DChanmode => {
                if ! self.channel_modes_d.contains(mode) {
                    self.channel_modes_d.push(mode);
                };
            },
        };
    }
    pub fn remove_channel_mode(&mut self, mode_type: ChannelModeType, mode: char, value: String) {
        match mode_type {
            ChannelModeType::NotAChanmode => (),
            ChannelModeType::AChanmode => {
                self.channel_modes_a.remove(&mode);
            },
            ChannelModeType::BChanmode => {
                self.channel_modes_b.remove(&mode);
            },
            ChannelModeType::CChanmode => {
                self.channel_modes_c.remove(&mode);
            },
            ChannelModeType::DChanmode => {
                if self.channel_modes_d.contains(mode) {
                    self.channel_modes_d = self.channel_modes_d.chars().filter(|c| *c != mode).collect();
                };
            },
        }
    }

    // TODO add user mode validation
    /// Add one or more mode flags to a user
    pub fn add_user_mode(&mut self, nick: String, mode: String) {
        if let Some(user_modes) = self.user_modes.get_mut(&nick) {
            mode.chars().for_each(|c| {
                if !user_modes.contains(c) {
                    user_modes.push(c);
                }
            });
        } else {
            self.user_modes.insert(nick.clone(), mode.clone());
        };

        // We need to iterate over all the chars and send a sync for each one
        // to stay compatible with quassels current behaviour
        // TODO this might actually be dumb can IRC even into mutiple modes at once?
        #[cfg(feature = "server")]
        if let Some(user_modes) = self.user_modes.get(&nick) {
            mode.chars().for_each(|c| {
                if !user_modes.contains(c) {
                    sync!("addUserMode", [nick.clone(), c.to_string()]);
                }
            });
        };
    }

    /// Remove one or more mode flags from a user
    pub fn remove_user_mode(&mut self, nick: String, mode: String) {
        if let Some(user_modes) = self.user_modes.get_mut(&nick) {
            mode.chars().for_each(|c| {
                *user_modes = user_modes.replace(c, "");
            });
        }

        #[cfg(feature = "server")]
        sync!("removeUserMode", [nick, mode]);
    }

    pub fn join_irc_users(&mut self, nicks: StringList, modes: StringList) {
        if nicks.len() != modes.len() {
            error!("number of nicks does not match number of modes");
        }

        #[cfg(feature = "server")]
        sync!("joinIrcUsers", [nicks.clone(), modes.clone()]);

        nicks
            .into_iter()
            .zip(modes)
            .for_each(|(nick, mode)| self.add_user_mode(nick, mode));
    }

    pub fn part(&mut self, nick: String) {
        match self.user_modes.remove(&nick) {
            Some(_) => (),
            None => warn!("tried to remove a user that is not joined to the channel"),
        }

        if self.user_modes.len() == 0
        /* nick.is_me() */
        {
            // TODO Clean up channel and delete
        }
    }

    pub fn set_user_modes(&mut self, nick: String, modes: String) {
        #[cfg(feature = "server")]
        sync!("setUserModes", [nick.clone(), modes.clone()]);

        *self.user_modes.entry(nick).or_default() = modes;
    }
}

#[cfg(feature = "client")]
impl crate::message::StatefulSyncableClient for IrcChannel {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            // "addChannelMode" => {
            //     let mode: String = get_param!(msg);
            //     self.add_channel_mode(mode.chars().next().unwrap(), get_param!(msg));
            // }
            // "removeChannelMode" => {
            //     let mode: String = get_param!(msg);
            //     self.remove_channel_mode(mode.chars().next().unwrap(), get_param!(msg));
            // }
            "addUserMode" => self.add_user_mode(get_param!(msg), get_param!(msg)),
            "removeUserMode" => self.remove_user_mode(get_param!(msg), get_param!(msg)),
            "joinIrcUsers" => self.join_irc_users(get_param!(msg), get_param!(msg)),
            "part" => self.part(get_param!(msg)),
            "setEncrypted" => self.set_encrypted(get_param!(msg)),
            "setPassword" => self.set_password(get_param!(msg)),
            "setTopic" => self.set_topic(get_param!(msg)),
            "setUserModes" => self.set_user_modes(get_param!(msg), get_param!(msg)),
            _ => (),
        }
    }

    /// Not Implemented for this type
    fn request_update(&mut self)
    where
        Self: Sized,
    {
    }
}

#[cfg(feature = "server")]
impl crate::message::StatefulSyncableServer for IrcChannel {
    /// Not Implemented for this type
    fn request_update(&mut self, _param: <IrcChannel as NetworkMap>::Item)
    where
        Self: Sized,
    {
    }
}

impl Syncable for IrcChannel {
    const CLASS: Class = Class::IrcChannel;
}

impl NetworkMap for Vec<IrcChannel> {
    type Item = VariantMap;

    fn to_network_map(&self) -> Self::Item {
        let mut channelmodes: VariantList = Vec::with_capacity(self.len());
        let mut usermodes: VariantList = Vec::with_capacity(self.len());
        let mut name: VariantList = Vec::with_capacity(self.len());
        let mut topic: VariantList = Vec::with_capacity(self.len());
        let mut password: VariantList = Vec::with_capacity(self.len());
        let mut encrypted: VariantList = Vec::with_capacity(self.len());

        let mut map = VariantMap::new();

        self.iter().for_each(|item| {
            channelmodes.push(Variant::VariantMap({
                let mut map = VariantMap::new();

                map.insert(
                    s!("A"),
                    Variant::VariantMap(
                        item.channel_modes_a
                            .iter()
                            .map(|(k, v)| (k.to_string(), Variant::StringList(v.clone())))
                            .collect(),
                    ),
                );
                map.insert(
                    s!("B"),
                    Variant::VariantMap(
                        item.channel_modes_b
                            .iter()
                            .map(|(k, v)| (k.to_string(), Variant::String(v.clone())))
                            .collect(),
                    ),
                );
                map.insert(
                    s!("C"),
                    Variant::VariantMap(
                        item.channel_modes_c
                            .iter()
                            .map(|(k, v)| (k.to_string(), Variant::String(v.clone())))
                            .collect(),
                    ),
                );
                map.insert(s!("D"), Variant::String(item.channel_modes_d.clone()));

                map
            }));

            usermodes.push(Variant::VariantMap(
                item.user_modes
                    .iter()
                    .map(|(k, v)| (k.clone(), Variant::String(v.clone())))
                    .collect(),
            ));
            name.push(Variant::String(item.name.clone()));
            topic.push(Variant::String(item.topic.clone()));
            password.push(Variant::String(item.password.clone()));
            encrypted.push(Variant::bool(item.encrypted));
        });

        map.insert(
            String::from("ChanModes"),
            Variant::VariantList(channelmodes),
        );
        map.insert(String::from("UserModes"), Variant::VariantList(usermodes));
        map.insert(String::from("name"), Variant::VariantList(name));
        map.insert(String::from("topic"), Variant::VariantList(topic));
        map.insert(String::from("password"), Variant::VariantList(password));
        map.insert(String::from("encrypted"), Variant::VariantList(encrypted));

        map
    }

    fn from_network_map(input: &mut Self::Item) -> Self {
        let marker: VariantList =
            std::convert::TryInto::try_into(input.get("name").unwrap()).unwrap();

        let mut res = Vec::new();
        for _ in 0..marker.len() {
            res.push(IrcChannel::from_network_map(input));
        }

        return res;
    }
}

impl NetworkMap for IrcChannel {
    type Item = VariantMap;

    fn to_network_map(&self) -> Self::Item {
        let mut res = VariantMap::new();

        res.insert(
            s!("ChanModes"),
            Variant::VariantList({
                let mut map = VariantMap::new();

                map.insert(
                    s!("A"),
                    Variant::VariantMap(
                        self.channel_modes_a
                            .iter()
                            .map(|(k, v)| (k.to_string(), Variant::StringList(v.clone())))
                            .collect(),
                    ),
                );
                map.insert(
                    s!("B"),
                    Variant::VariantMap(
                        self.channel_modes_b
                            .iter()
                            .map(|(k, v)| (k.to_string(), Variant::String(v.clone())))
                            .collect(),
                    ),
                );
                map.insert(
                    s!("C"),
                    Variant::VariantMap(
                        self.channel_modes_c
                            .iter()
                            .map(|(k, v)| (k.to_string(), Variant::String(v.clone())))
                            .collect(),
                    ),
                );
                map.insert(s!("D"), Variant::String(self.channel_modes_d.clone()));

                vec![Variant::VariantMap(map)]
            }),
        );

        res.insert(
            s!("UserModes"),
            Variant::VariantList(vec![Variant::VariantMap(
                self.user_modes
                    .iter()
                    .map(|(k, v)| (k.clone(), Variant::String(v.clone())))
                    .collect(),
            )]),
        );
        res.insert(
            s!("name"),
            Variant::VariantList(vec![Variant::String(self.name.clone())]),
        );
        res.insert(
            s!("topic"),
            Variant::VariantList(vec![Variant::String(self.topic.clone())]),
        );
        res.insert(
            s!("password"),
            Variant::VariantList(vec![Variant::String(self.password.clone())]),
        );
        res.insert(
            s!("encrypted"),
            Variant::VariantList(vec![Variant::bool(self.encrypted.clone())]),
        );

        res
    }
    fn from_network_map(input: &mut Self::Item) -> Self {
        let mut chanmodes: VariantMap = match_variant!(
            match_variant!(input.get_mut("ChanModes").unwrap(), Variant::VariantList).remove(0),
            Variant::VariantMap
        );

        Self {
            channel_modes_a: match_variant!(chanmodes.remove("A").unwrap(), Variant::VariantMap)
                .into_iter()
                .map(|(mut k, v)| (k.remove(0), match_variant!(v, Variant::StringList)))
                .collect(),
            channel_modes_b: match_variant!(chanmodes.remove("B").unwrap(), Variant::VariantMap)
                .into_iter()
                .map(|(mut k, v)| (k.remove(0), match_variant!(v, Variant::String)))
                .collect(),
            channel_modes_c: match_variant!(chanmodes.remove("C").unwrap(), Variant::VariantMap)
                .into_iter()
                .map(|(mut k, v)| (k.remove(0), match_variant!(v, Variant::String)))
                .collect(),
            channel_modes_d: match_variant!(chanmodes.remove("D").unwrap(), Variant::String),
            user_modes: VariantMap::try_from(
                match_variant!(input.get_mut("UserModes").unwrap(), Variant::VariantList).remove(0),
            )
            .unwrap()
            .into_iter()
            .map(|(k, v)| (k, v.try_into().unwrap()))
            .collect(),
            name: match_variant!(input.get_mut("name").unwrap(), Variant::VariantList)
                .remove(0)
                .try_into()
                .unwrap(),
            topic: match_variant!(input.get_mut("topic").unwrap(), Variant::VariantList)
                .remove(0)
                .try_into()
                .unwrap(),
            password: match_variant!(input.get_mut("password").unwrap(), Variant::VariantList)
                .remove(0)
                .try_into()
                .unwrap(),
            encrypted: match_variant!(input.get_mut("encrypted").unwrap(), Variant::VariantList)
                .remove(0)
                .try_into()
                .unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_network() -> VariantMap {
        map! {
            s!("encrypted") => Variant::VariantList(
                vec![
                    Variant::bool(
                        false,
                    ),
                ],
            ),
            s!("topic") => Variant::VariantList(
                vec![
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("password") => Variant::VariantList(
                vec![
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("ChanModes") => Variant::VariantList(
                vec![
                    Variant::VariantMap(map!
                        {
                            s!("B") => Variant::VariantMap(map!
                                {},
                            ),
                            s!("D") => Variant::String(
                                s!("tCnT"),
                            ),
                            s!("C") => Variant::VariantMap(map!
                                {
                                    s!("j") => Variant::String(
                                        s!("5:1"),
                                    ),
                                    s!("x") => Variant::String(
                                        s!("10:5"),
                                    ),
                                    s!("f") => Variant::String(
                                        s!("30:5"),
                                    ),
                                    s!("F") => Variant::String(
                                        s!("5:60"),
                                    ),
                                },
                            ),
                            s!("A") => Variant::VariantMap(map! {
                                s!("b") => Variant::StringList(vec![s!("*!*@test"), s!("*!*@test2")]),
                            }),
                        },
                    ),
                ],
            ),
            s!("UserModes") => Variant::VariantList(
                vec![
                    Variant::VariantMap(map!
                        {
                            s!("audron") => Variant::String(
                                s!("o"),
                            ),
                            s!("audron_") => Variant::String(
                                s!(""),
                            ),
                        },
                    ),
                ],
            ),
            s!("name") => Variant::VariantList(
                vec![
                    Variant::String(
                        s!("#audron-test"),
                    ),
                ],
            )
        }
    }
    fn get_runtime() -> IrcChannel {
        IrcChannel {
            channel_modes_a: map! { 'b' => vec![s!("*!*@test"), s!("*!*@test2")] },
            channel_modes_b: map! {},
            channel_modes_c: map! { 'j' => s!("5:1"), 'x' => s!("10:5"), 'f' => s!("30:5"), 'F' => s!("5:60") },
            channel_modes_d: s!("tCnT"),
            user_modes: map! { s!("audron") => s!("o"), s!("audron_") => s!("") },
            name: s!("#audron-test"),
            topic: s!(""),
            password: s!(""),
            encrypted: false,
        }
    }

    #[test]
    fn ircchannel_to_network() {
        assert_eq!(get_runtime().to_network_map(), get_network())
    }

    #[test]
    fn ircchannel_from_network() {
        assert_eq!(
            IrcChannel::from_network_map(&mut get_network()),
            get_runtime()
        )
    }

    #[test]
    fn add_user_mode() {
        let mut base = get_runtime();
        let mut res = get_runtime();
        res.user_modes = map! { s!("audron") => s!("oh"), s!("audron_") => s!("") };

        base.add_user_mode(s!("audron"), s!("h"));
        assert_eq!(res, base);
        base.add_user_mode(s!("audron"), s!("o"));
        assert_eq!(res, base);

        res.user_modes =
            map! { s!("audron") => s!("oh"), s!("audron_") => s!(""), s!("test") => s!("h") };
        base.add_user_mode(s!("test"), s!("h"));
        assert_eq!(res, base);
    }
}
