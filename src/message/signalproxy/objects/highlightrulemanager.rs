use libquassel_derive::{NetworkList, NetworkMap};

use crate::message::signalproxy::translation::{Network, NetworkMap};

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[derive(Debug, Clone, PartialEq, NetworkList)]
pub struct HighlightRuleManager {
    #[network(rename = "HighlightRuleList", variant = "VariantMap", network, map)]
    highlight_rule_list: Vec<HighlightRule>,
    #[network(rename = "highlightNick", variant = "i32", network)]
    highlight_nick: HighlightNickType,
    #[network(rename = "nicksCaseSensitive")]
    nicks_case_sensitive: bool,
}

#[derive(Debug, Clone, PartialEq, NetworkMap)]
#[network(repr = "maplist")]
pub struct HighlightRule {
    id: i32,
    #[network(variant = "StringList")]
    name: String,
    #[network(rename = "isRegEx")]
    is_regex: bool,
    #[network(rename = "isCaseSensitive")]
    is_case_sensitive: bool,
    #[network(rename = "isEnabled")]
    is_enabled: bool,
    #[network(rename = "isInverse")]
    is_inverse: bool,
    #[network(variant = "StringList")]
    sender: String,
    #[network(variant = "StringList")]
    channel: String,
}

#[derive(Debug, Clone, PartialEq, FromPrimitive, ToPrimitive)]
pub enum HighlightNickType {
    NoNick = 0x00,
    CurrentNick = 0x01,
    AllNicks = 0x02,
}

impl crate::message::signalproxy::Network for HighlightNickType {
    type Item = i32;

    fn to_network(&self) -> Self::Item {
        self.to_i32().unwrap()
    }

    fn from_network(input: &mut Self::Item) -> Self {
        Self::from_i32(*input).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::signalproxy::translation::NetworkList;
    use crate::primitive::{Variant, VariantList};

    use pretty_assertions::assert_eq;

    fn get_network() -> VariantList {
        vec![
            Variant::ByteArray(s!("HighlightRuleList")),
            Variant::VariantMap(map! {
                s!("isInverse") => Variant::VariantList(vec![Variant::bool(false)]),
                s!("isEnabled") => Variant::VariantList(vec![Variant::bool(true)]),
                s!("channel") => Variant::StringList(vec![s!("#test")]),
                s!("sender") => Variant::StringList(vec![s!("testuser")]),
                s!("isCaseSensitive") => Variant::VariantList(vec![Variant::bool(false)]),
                s!("isRegEx") => Variant::VariantList(vec![Variant::bool(false)]),
                s!("name") => Variant::StringList(vec![s!("testrule")]),
                s!("id") => Variant::VariantList(vec![Variant::i32(1)]),
            }),
            Variant::ByteArray(s!("highlightNick")),
            Variant::i32(1),
            Variant::ByteArray(s!("nicksCaseSensitive")),
            Variant::bool(false),
        ]
    }

    fn get_runtime() -> HighlightRuleManager {
        HighlightRuleManager {
            highlight_rule_list: vec![HighlightRule {
                id: 1,
                name: s!("testrule"),
                is_regex: false,
                is_case_sensitive: false,
                is_enabled: true,
                is_inverse: false,
                sender: s!("testuser"),
                channel: s!("#test"),
            }],
            highlight_nick: HighlightNickType::CurrentNick,
            nicks_case_sensitive: false,
        }
    }

    #[test]
    fn highlightrulemanager_to_network() {
        assert_eq!(get_runtime().to_network_list(), get_network())
    }

    #[test]
    fn highlightrulemanager_from_network() {
        assert_eq!(
            HighlightRuleManager::from_network_list(&mut get_network()),
            get_runtime()
        )
    }
}
