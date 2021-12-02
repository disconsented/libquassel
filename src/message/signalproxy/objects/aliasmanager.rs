use libquassel_derive::{NetworkList, NetworkMap};

#[allow(unused_imports)]
use crate::message::StatefulSyncableClient;
#[allow(unused_imports)]
use crate::message::StatefulSyncableServer;

use crate::message::Syncable;

use crate::message::signalproxy::translation::NetworkMap;

#[allow(unused_imports)]
use crate::primitive::VariantMap;

/// AliasManager
/// keeps a list of all registered aliases
/// syncable
#[derive(Clone, Debug, std::cmp::PartialEq, NetworkList, NetworkMap)]
pub struct AliasManager {
    #[network(rename = "Aliases", variant = "VariantMap", network, map)]
    pub aliases: Vec<Alias>,
}

impl AliasManager {
    pub fn add_alias(&mut self, alias: Alias) {
        // TODO check if name is equal
        if !self.aliases.contains(&alias) {
            self.aliases.push(alias)
        }
    }
}

#[cfg(feature = "client")]
impl StatefulSyncableClient for AliasManager {}

#[cfg(feature = "server")]
impl StatefulSyncableServer for AliasManager {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "addAlias" => self.add_alias(Alias::from_network_map(
                &mut VariantMap::try_from(msg.params.pop().unwrap()).unwrap(),
            )),
            _ => (),
        }
    }
}

impl Syncable for AliasManager {
    const CLASS: &'static str = "AliasManager";
}

/// Alias
/// Represents a signle alias
#[derive(Clone, Debug, std::cmp::PartialEq, NetworkMap)]
#[network(repr = "maplist")]
pub struct Alias {
    #[network(rename = "names", variant = "StringList")]
    pub name: String,
    #[network(rename = "expansions", variant = "StringList")]
    pub expansion: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::signalproxy::translation::NetworkList;

    use crate::primitive::{Variant, VariantList};

    fn get_src() -> AliasManager {
        AliasManager {
            aliases: vec![
                Alias {
                    name: s!("j"),
                    expansion: s!("/join $0"),
                },
                Alias {
                    name: s!("ns"),
                    expansion: s!("/msg nickserv $0"),
                },
            ],
        }
    }

    fn get_dest() -> VariantList {
        vec![
            Variant::ByteArray(s!("Aliases")),
            Variant::VariantMap(map! {
                s!("names") => Variant::StringList(
                    vec![
                        s!("j"),
                        s!("ns"),
                    ],
                ),
                s!("expansions") => Variant::StringList(
                    vec![
                        s!("/join $0"),
                        s!("/msg nickserv $0"),
                    ],
                ),
            }),
        ]
    }

    // #[bench]
    // fn alias_to_network(b: &mut test::Bencher) {
    //     b.iter(|| test::black_box(get_src()).to_network())
    // }

    // #[bench]
    // fn alias_from_network(b: &mut test::Bencher) {
    //     b.iter(|| AliasManager::from_network(&mut test::black_box(get_dest())))
    // }

    #[test]
    fn aliasmanager_to_network() {
        assert_eq!(get_src().to_network_list(), get_dest())
    }

    #[test]
    fn aliasmanager_from_network() {
        assert_eq!(AliasManager::from_network_list(&mut get_dest()), get_src())
    }
}
