use crate::primitive::{StringList, Variant, VariantMap};

use crate::message::signalproxy::Network;

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct AliasManager {
    pub aliases: Vec<Alias>,
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct Alias {
    name: String,
    expansion: String,
}

impl Alias {
    fn from_network_internal(input: &VariantMap) -> Vec<Self> {
        let names = match_variant!(input.get("names").unwrap(), Variant::StringList);
        let expansions = match_variant!(input.get("expansions").unwrap(), Variant::StringList);

        return names
            .iter()
            .zip(expansions)
            .map(|(name, expansion)| Alias {
                name: name.clone(),
                expansion,
            })
            .collect();
    }
}

impl Network for Alias {
    type Item = VariantMap;

    fn to_network(&self) -> Self::Item {
        let mut map = VariantMap::new();
        map.insert(s!("names"), Variant::StringList(vec![self.name.clone()]));
        map.insert(
            s!("expansions"),
            Variant::StringList(vec![self.expansion.clone()]),
        );

        return map;
    }

    fn from_network(input: &mut Self::Item) -> Self {
        Alias::from_network_internal(&input)[0].clone()
    }
}

impl Network for AliasManager {
    type Item = VariantMap;

    fn to_network(&self) -> Self::Item {
        let (names, expansions) = self.aliases.iter().fold(
            (StringList::new(), StringList::new()),
            |(mut names, mut expansions), alias| {
                names.push(alias.name.clone());
                expansions.push(alias.expansion.clone());
                return (names, expansions);
            },
        );

        let mut map = VariantMap::new();
        map.insert(s!("names"), Variant::StringList(names));
        map.insert(s!("expansions"), Variant::StringList(expansions));

        return map;
    }

    fn from_network(input: &mut Self::Item) -> Self {
        let names = match_variant!(input.get("names").unwrap(), Variant::StringList);
        let expansions = match_variant!(input.get("expansions").unwrap(), Variant::StringList);

        AliasManager {
            aliases: names
                .iter()
                .zip(expansions)
                .map(|(name, expansion)| Alias {
                    name: name.clone(),
                    expansion,
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn get_dest() -> VariantMap {
        map! {
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
        }
    }

    #[test]
    fn aliasmanager_to_network() {
        assert_eq!(get_src().to_network(), get_dest())
    }

    #[test]
    fn aliasmanager_from_network() {
        assert_eq!(AliasManager::from_network(&mut get_dest()), get_src())
    }
}

// impl AliasManager {
//     /// Client to Server
//     ///
//     /// Replaces all properties of the object with the content of the
//     /// "properties" parameter. This parameter is in network representation.
//     ///
//     fn request_update(self: &mut Self, properties: VariantMap) {
//         self.update(properties);
//     }

//     /// Server to Client
//     fn add_alias(self: &mut Self, name: String, expansion: String) {
//         self.aliases.push(Alias { name, expansion });
//     }

//     /// Server to Client
//     ///
//     /// Replaces all properties of the object with the content of the
//     /// "properties" parameter. This parameter is in network representation.
//     ///
//     fn update(self: &mut Self, properties: VariantMap) {
//         let mut alias: Vec<Alias> = Vec::new();

//         // for (i, name) in match_variant!(properties[&"Aliases".to_string()], Variant::String) {
//         //     alias.push(Alias {
//         //         name,
//         //         expansion: match_variant!(properties["Aliases"], Variant::String)["expansions"][i],
//         //     })
//         // }

//         self.aliases = alias
//     }
// }
