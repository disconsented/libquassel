use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;

use crate::message::NetworkMap;
use crate::primitive::{StringList, Variant, VariantList, VariantMap};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct IrcChannel {
    pub channel_modes_a: HashMap<char, StringList>,
    pub channel_modes_b: HashMap<char, String>,
    pub channel_modes_c: HashMap<char, String>,
    pub channel_modes_d: String,
    // pub channel_modes: HashMap<char, ChannelMode>,
    pub user_modes: HashMap<String, String>,
    pub name: String,
    pub topic: String,
    pub password: String,
    pub encrypted: bool,
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum ChannelMode {
//     A(char, StringList),
//     B(char, String),
//     C(char, String),
//     D(char),
// }

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
}
