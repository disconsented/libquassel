#[allow(unused_imports)]
use libquassel_derive::sync;
use libquassel_derive::{NetworkList, NetworkMap, Setters};

#[allow(unused_imports)]
use crate::message::StatefulSyncableClient;
#[allow(unused_imports)]
use crate::message::StatefulSyncableServer;

use crate::message::Syncable;

#[allow(unused_imports)]
use crate::message::signalproxy::translation::NetworkMap;

#[derive(Debug, Clone, PartialEq, NetworkMap, NetworkList, Setters)]
pub struct Identity {
    #[quassel(name = "identityId")]
    identity_id: i32,
    #[quassel(name = "identityName")]
    identity_name: String,
    #[quassel(name = "realName")]
    real_name: String,
    #[quassel(name = "nicks")]
    #[network(type = "StringList")]
    nicks: Vec<String>,
    #[quassel(name = "awayNick")]
    away_nick: String,
    #[quassel(name = "awayNickEnabled")]
    away_nick_enabled: bool,
    #[quassel(name = "awayReason")]
    away_reason: String,
    #[quassel(name = "awayReasonEnabled")]
    away_reason_enabled: bool,
    #[quassel(name = "autoAwayEnabled")]
    auto_away_enabled: bool,
    #[quassel(name = "autoAwayTime")]
    auto_away_time: i32,
    #[quassel(name = "autoAwayReason")]
    auto_away_reason: String,
    #[quassel(name = "autoAwayReasonEnabled")]
    auto_away_reason_enabled: bool,
    #[quassel(name = "detachAwayEnabled")]
    detach_away_enabled: bool,
    #[quassel(name = "detachAwayReason")]
    detach_away_reason: String,
    #[quassel(name = "detachAwayReasonEnabled")]
    detach_away_reason_enabled: bool,
    #[quassel(name = "ident")]
    ident: String,
    #[quassel(name = "kickReason")]
    kick_reason: String,
    #[quassel(name = "partReason")]
    part_reason: String,
    #[quassel(name = "quitReason")]
    quit_reason: String,
}

impl Identity {
    pub fn copy_from(&mut self, other: Identity) {
        #[cfg(feature = "server")]
        sync!("copyFrom", [other.to_network_map()]);

        *self = other;
    }
}

#[cfg(feature = "client")]
impl StatefulSyncableClient for Identity {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "copyFrom" => self.copy_from(Identity::from_network_map(&mut get_param!(msg))),
            "setAutoAwayEnabled" => self.set_auto_away_enabled(get_param!(msg)),
            "setAutoAwayReason" => self.set_auto_away_reason(get_param!(msg)),
            "setAutoAwayReasonEnabled" => self.set_auto_away_reason_enabled(get_param!(msg)),
            "setAutoAwayTime" => self.set_auto_away_time(get_param!(msg)),
            "setAwayNick" => self.set_away_nick(get_param!(msg)),
            "setAwayNickEnabled" => self.set_away_nick_enabled(get_param!(msg)),
            "setAwayReason" => self.set_away_reason(get_param!(msg)),
            "setAwayReasonEnabled" => self.set_away_reason_enabled(get_param!(msg)),
            "setDetachAwayEnabled" => self.set_detach_away_enabled(get_param!(msg)),
            "setDetachAwayReason" => self.set_detach_away_reason(get_param!(msg)),
            "setDetachAwayReasonEnabled" => self.set_detach_away_reason_enabled(get_param!(msg)),
            "setId" => self.set_identity_id(get_param!(msg)),
            "setIdent" => self.set_ident(get_param!(msg)),
            "setIdentityName" => self.set_identity_name(get_param!(msg)),
            "setKickReason" => self.set_kick_reason(get_param!(msg)),
            "setNicks" => self.set_nicks(get_param!(msg)),
            "setPartReason" => self.set_part_reason(get_param!(msg)),
            "setQuitReason" => self.set_quit_reason(get_param!(msg)),
            "setRealName" => self.set_real_name(get_param!(msg)),
            _ => (),
        }
    }
}

#[cfg(feature = "server")]
impl StatefulSyncableServer for Identity {}

impl Syncable for Identity {
    const CLASS: &'static str = "Identity";

    fn send_sync(&self, function: &str, params: crate::primitive::VariantList) {
        crate::message::signalproxy::SYNC_PROXY.get().unwrap().sync(
            Self::CLASS,
            Some(&self.identity_id.to_string()),
            function,
            params,
        );
    }
}
