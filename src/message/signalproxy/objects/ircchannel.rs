use std::collections::HashMap;

use crate::primitive::{StringList, Variant, VariantMap};

#[allow(unused_imports)]
use crate::message::signalproxy::Network;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct IrcChannel {
    channel_modes_a: HashMap<char, StringList>,
    channel_modes_b: HashMap<char, String>,
    channel_modes_c: HashMap<char, String>,
    channel_modes_d: String,
    user_modes: HashMap<String, String>,
    name: String,
    topic: String,
    password: String,
    encrypted: bool,
}

impl Network for IrcChannel {
    type Item = VariantMap;

    fn to_network(&self) -> Self::Item {
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
    fn from_network(input: &mut Self::Item) -> Self {
        Self {
            channel_modes_a: match_variant!(
                match_variant!(
                    match_variant!(input.get("ChanModes").unwrap(), Variant::VariantList)[0],
                    Variant::VariantMap
                )
                .get("B")
                .unwrap(),
                Variant::VariantMap
            )
            .iter()
            .map(|(k, v)| {
                (
                    k.chars().nth(0).unwrap(),
                    match_variant!(v, Variant::StringList),
                )
            })
            .collect(),
            channel_modes_b: match_variant!(
                match_variant!(
                    match_variant!(input.get("ChanModes").unwrap(), Variant::VariantList)[0],
                    Variant::VariantMap
                )
                .get("B")
                .unwrap(),
                Variant::VariantMap
            )
            .iter()
            .map(|(k, v)| {
                (
                    k.chars().nth(0).unwrap(),
                    match_variant!(v, Variant::String),
                )
            })
            .collect(),
            channel_modes_c: match_variant!(
                match_variant!(
                    match_variant!(input.get("ChanModes").unwrap(), Variant::VariantList)[0],
                    Variant::VariantMap
                )
                .get("C")
                .unwrap(),
                Variant::VariantMap
            )
            .iter()
            .map(|(k, v)| {
                (
                    k.chars().nth(0).unwrap(),
                    match_variant!(v, Variant::String),
                )
            })
            .collect(),
            channel_modes_d: match_variant!(
                match_variant!(
                    match_variant!(input.get("ChanModes").unwrap(), Variant::VariantList)[0],
                    Variant::VariantMap
                )
                .get("D")
                .unwrap(),
                Variant::String
            ),
            user_modes: match_variant!(
                match_variant!(input.get("UserModes").unwrap(), Variant::VariantList)[0],
                Variant::VariantMap
            )
            .iter()
            .map(|(k, v)| (k.clone(), match_variant!(v, Variant::String)))
            .collect(),
            name: match_variant!(
                match_variant!(input.get("name").unwrap(), Variant::VariantList)[0],
                Variant::String
            ),
            topic: match_variant!(
                match_variant!(input.get("topic").unwrap(), Variant::VariantList)[0],
                Variant::String
            ),
            password: match_variant!(
                match_variant!(input.get("password").unwrap(), Variant::VariantList)[0],
                Variant::String
            ),
            encrypted: match_variant!(
                match_variant!(input.get("encrypted").unwrap(), Variant::VariantList)[0],
                Variant::bool
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_network() -> VariantMap {
        VariantMap::from(map! {
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
                            s!("A") => Variant::VariantMap(map!
                                {},
                            ),
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
        })
    }
    fn get_runtime() -> IrcChannel {
        IrcChannel {
            channel_modes_a: map! {},
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
        assert_eq!(get_runtime().to_network(), get_network())
    }

    #[test]
    fn ircchannel_from_network() {
        assert_eq!(IrcChannel::from_network(&mut get_network()), get_runtime())
    }
}
