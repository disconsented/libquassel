use libquassel_derive::sync;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

use libquassel_derive::{NetworkList, NetworkMap};

use crate::message::signalproxy::translation::{Network, NetworkMap};

#[allow(unused_imports)]
use crate::message::StatefulSyncableClient;
#[allow(unused_imports)]
use crate::message::StatefulSyncableServer;

use crate::message::Syncable;

#[derive(Debug, Clone, PartialEq, NetworkList, NetworkMap)]
pub struct HighlightRuleManager {
    #[network(rename = "HighlightRuleList", variant = "VariantMap", network, map)]
    highlight_rule_list: Vec<HighlightRule>,
    #[network(rename = "highlightNick", variant = "i32", network)]
    highlight_nick: HighlightNickType,
    #[network(rename = "nicksCaseSensitive")]
    nicks_case_sensitive: bool,
}

impl HighlightRuleManager {
    /// Get a reference to a specific highlight rule by ID.
    pub fn highlight_rule(&self, id: i32) -> Option<&HighlightRule> {
        if let Some(position) = self
            .highlight_rule_list
            .iter()
            .position(|rule| rule.id == id)
        {
            self.highlight_rule_list.get(position)
        } else {
            None
        }
    }

    /// Get a mutable reference to a specific highlight rule by ID.
    pub fn highlight_rule_mut(&mut self, id: i32) -> Option<&mut HighlightRule> {
        if let Some(position) = self
            .highlight_rule_list
            .iter()
            .position(|rule| rule.id == id)
        {
            self.highlight_rule_list.get_mut(position)
        } else {
            None
        }
    }

    pub fn request_remove_highlight_rule(&self, id: i32) {
        sync!("requestRemoveHighlightRule", [id])
    }

    pub fn request_toggle_highlight_rule(&self, id: i32) {
        sync!("requestToggleHighlightRule", [id])
    }

    pub fn request_add_highlight_rule(
        &self,
        id: i32,
        name: String,
        is_regex: bool,
        is_case_sensitive: bool,
        is_enabled: bool,
        is_inverse: bool,
        sender: String,
        channel: String,
    ) {
        sync!(
            "requestAddHighlightRule",
            [
                id,
                name,
                is_regex,
                is_case_sensitive,
                is_enabled,
                is_inverse,
                sender,
                channel
            ]
        )
    }

    pub fn request_set_highlight_nick(&self, nick: HighlightNickType) {
        sync!("requestSetHighlightNick", [nick.to_network()])
    }

    pub fn request_set_nicks_case_sensitive(&self, enabled: bool) {
        sync!("requestSetNicksCaseSensitive", [enabled])
    }

    pub fn remove_highlight_rule(&mut self, id: i32) {
        if let Some(position) = self
            .highlight_rule_list
            .iter()
            .position(|rule| rule.id == id)
        {
            self.highlight_rule_list.remove(position);
        }

        #[cfg(feature = "server")]
        sync!("removeHighlightRule", [id]);
    }

    pub fn toggle_highlight_rule(&mut self, id: i32) {
        if let Some(rule) = self.highlight_rule_mut(id) {
            rule.is_enabled = !rule.is_enabled;
        }

        #[cfg(feature = "server")]
        sync!("toggleHighlightRule", [id])
    }

    pub fn add_highlight_rule(&mut self, rule: HighlightRule) {
        #[cfg(feature = "server")]
        sync!(
            "addHighlightRule",
            [
                rule.id.clone(),
                rule.name.clone(),
                rule.is_regex.clone(),
                rule.is_case_sensitive.clone(),
                rule.is_enabled.clone(),
                rule.is_inverse.clone(),
                rule.sender.clone(),
                rule.channel.clone()
            ]
        );

        self.highlight_rule_list.push(rule);
    }

    pub fn set_highlight_nick(&mut self, nick: HighlightNickType) {
        #[cfg(feature = "server")]
        sync!("setHighlightNick", [nick.to_network()]);

        self.highlight_nick = nick;
    }

    pub fn set_nicks_case_sensitive(&mut self, enabled: bool) {
        #[cfg(feature = "server")]
        sync!("setNicksCaseSensitive", [enabled]);

        self.nicks_case_sensitive = enabled;
    }
}

#[cfg(feature = "client")]
impl StatefulSyncableClient for HighlightRuleManager {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "removeHighlightRule" => self.remove_highlight_rule(get_param!(msg)),
            "toggleHighlightRule" => self.toggle_highlight_rule(get_param!(msg)),
            "addHighlightRule" => self.add_highlight_rule(HighlightRule {
                id: get_param!(msg),
                name: get_param!(msg),
                is_regex: get_param!(msg),
                is_case_sensitive: get_param!(msg),
                is_enabled: get_param!(msg),
                is_inverse: get_param!(msg),
                sender: get_param!(msg),
                channel: get_param!(msg),
            }),
            "setHighlightNick" => {
                self.set_highlight_nick(HighlightNickType::from_network(&mut get_param!(msg)))
            }
            "setNicksCaseSensitive" => self.set_nicks_case_sensitive(get_param!(msg)),
            _ => (),
        }
    }
}

#[cfg(feature = "server")]
impl StatefulSyncableServer for HighlightRuleManager {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "requestRemoveHighlightRule" => self.remove_highlight_rule(get_param!(msg)),
            "requestToggleHighlightRule" => self.toggle_highlight_rule(get_param!(msg)),
            "requestAddHighlightRule" => self.add_highlight_rule(HighlightRule {
                id: get_param!(msg),
                name: get_param!(msg),
                is_regex: get_param!(msg),
                is_case_sensitive: get_param!(msg),
                is_enabled: get_param!(msg),
                is_inverse: get_param!(msg),
                sender: get_param!(msg),
                channel: get_param!(msg),
            }),
            "requestSetHighlightNick" => {
                self.set_highlight_nick(HighlightNickType::from_network(&mut get_param!(msg)))
            }
            "requestSetNicksCaseSensitive" => self.set_nicks_case_sensitive(get_param!(msg)),
            _ => (),
        }
    }
}

impl Syncable for HighlightRuleManager {
    const CLASS: &'static str = "HighlightRuleManager";
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
