use crate::primitive::{Variant, VariantMap};

#[derive(Debug, Clone)]
pub struct Identity {
    identity_id: i32,
    identity_name: String,
    real_name: String,
    nicks: Vec<String>,
    away_nick: String,
    away_nick_enabled: bool,
    away_reason: String,
    away_reason_enabled: bool,
    auto_away_enabled: bool,
    auto_away_time: i32,
    auto_away_reason: String,
    auto_away_reason_enabled: bool,
    detach_away_enabled: bool,
    detach_away_reason: String,
    detach_away_reason_enabled: bool,
    ident: String,
    kick_reason: String,
    part_reason: String,
    quit_reason: String,
}

impl From<VariantMap> for Identity {
    fn from(input: VariantMap) -> Self {
        Identity {
            identity_id: match_variant!(input.get("identityId").unwrap(), Variant::i32),
            identity_name: match_variant!(input.get("identityName").unwrap(), Variant::String),
            real_name: match_variant!(input.get("realName").unwrap(), Variant::String),
            nicks: match_variant!(input.get("nicks").unwrap(), Variant::StringList),
            away_nick: match_variant!(input.get("awayNick").unwrap(), Variant::String),
            away_nick_enabled: match_variant!(input.get("awayNickEnabled").unwrap(), Variant::bool),
            away_reason: match_variant!(input.get("awayReason").unwrap(), Variant::String),
            away_reason_enabled: match_variant!(
                input.get("awayReasonEnabled").unwrap(),
                Variant::bool
            ),
            auto_away_enabled: match_variant!(input.get("autoAwayEnabled").unwrap(), Variant::bool),
            auto_away_time: match_variant!(input.get("autoAwayTime").unwrap(), Variant::i32),
            auto_away_reason: match_variant!(input.get("autoAwayReason").unwrap(), Variant::String),
            auto_away_reason_enabled: match_variant!(
                input.get("autoAwayReasonEnabled").unwrap(),
                Variant::bool
            ),
            detach_away_enabled: match_variant!(
                input.get("detachAwayEnabled").unwrap(),
                Variant::bool
            ),
            detach_away_reason: match_variant!(
                input.get("detachAwayReason").unwrap(),
                Variant::String
            ),
            detach_away_reason_enabled: match_variant!(
                input.get("detachAwayReasonEnabled").unwrap(),
                Variant::bool
            ),
            ident: match_variant!(input.get("ident").unwrap(), Variant::String),
            kick_reason: match_variant!(input.get("kickReason").unwrap(), Variant::String),
            part_reason: match_variant!(input.get("partReason").unwrap(), Variant::String),
            quit_reason: match_variant!(input.get("quitReason").unwrap(), Variant::String),
        }
    }
}

impl Into<std::collections::HashMap<String, Variant>> for Identity {
    fn into(self) -> VariantMap {
        let mut res = VariantMap::with_capacity(19);

        res.insert("identityId".to_string(), Variant::i32(self.identity_id));
        res.insert(
            "identityName".to_string(),
            Variant::String(self.identity_name),
        );
        res.insert("realName".to_string(), Variant::String(self.real_name));
        res.insert("nicks".to_string(), Variant::StringList(self.nicks));
        res.insert("awayNick".to_string(), Variant::String(self.away_nick));
        res.insert(
            "awayNickEnabled".to_string(),
            Variant::bool(self.away_nick_enabled),
        );
        res.insert("awayReason".to_string(), Variant::String(self.away_reason));
        res.insert(
            "awayReasonEnabled".to_string(),
            Variant::bool(self.away_reason_enabled),
        );
        res.insert(
            "autoAwayEnabled".to_string(),
            Variant::bool(self.auto_away_enabled),
        );
        res.insert(
            "autoAwayTime".to_string(),
            Variant::i32(self.auto_away_time),
        );
        res.insert(
            "autoAwayReason".to_string(),
            Variant::String(self.auto_away_reason),
        );
        res.insert(
            "detachAwayEnabled".to_string(),
            Variant::bool(self.detach_away_enabled),
        );
        res.insert(
            "detachAwayReason".to_string(),
            Variant::String(self.detach_away_reason),
        );
        res.insert(
            "detachAwayReasonEnabled".to_string(),
            Variant::bool(self.detach_away_reason_enabled),
        );
        res.insert("ident".to_string(), Variant::String(self.ident));
        res.insert("kickReason".to_string(), Variant::String(self.kick_reason));
        res.insert("partReason".to_string(), Variant::String(self.part_reason));
        res.insert("quitReason".to_string(), Variant::String(self.quit_reason));

        res
    }
}
