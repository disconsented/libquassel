use libquassel_derive::Network;

#[derive(Clone, Debug, std::cmp::PartialEq, Network)]
#[network(repr = "list")]
pub struct AliasManager {
    #[network(rename = "Aliases", variant = "VariantMap", network)]
    pub aliases: Vec<Alias>,
}

#[derive(Clone, Debug, std::cmp::PartialEq, Network)]
#[network(repr = "maplist")]
pub struct Alias {
    #[network(rename = "names", variant = "StringList")]
    name: String,
    #[network(rename = "expansions", variant = "StringList")]
    expansion: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::signalproxy::translation::Network;

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
