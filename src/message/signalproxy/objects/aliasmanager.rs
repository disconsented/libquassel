use crate::primitive::{Variant, VariantMap};

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct AliasManager {
    pub aliases: Vec<Alias>,
}

#[derive(Clone, Debug, std::cmp::PartialEq)]
pub struct Alias {
    name: String,
    expansion: String,
}

// impl AliasManager {
//     /// Client to Server
//     ///
//     /// Replaces all properties of the object with the content of the
//     /// "properties" parameter. This parameter is in network representation.
//     ///
//     fn request_update(self: Self, properties: VariantMap) {
//         self.update(properties);
//     }
//
//     /// Server to Client
//     fn add_alias(self: Self, name: String, expansion: String) {
//         self.aliases.push(Alias { name, expansion });
//     }
//
//     /// Server to Client
//     ///
//     /// Replaces all properties of the object with the content of the
//     /// "properties" parameter. This parameter is in network representation.
//     ///
//     fn update(self: Self, properties: VariantMap) {
//         let mut alias: Vec<Alias> = Vec::new();
//
//         // for (i, name) in match_variant!(properties[&"Aliases".to_string()], Variant::String) {
//         //     alias.push(Alias {
//         //         name,
//         //         expansion: match_variant!(properties["Aliases"], Variant::String)["expansions"][i],
//         //     })
//         // }
//
//         self.aliases = alias
//     }
// }
