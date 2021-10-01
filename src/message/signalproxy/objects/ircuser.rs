use crate::primitive::{DateTime, StringList};

#[allow(unused_imports)]
use crate::message::signalproxy::Network;
use libquassel_derive::NetworkMap;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, NetworkMap)]
#[network(repr = "maplist")]
pub struct IrcUser {
    pub user: String,
    pub host: String,
    pub nick: String,
    #[network(rename = "realName")]
    pub real_name: String,
    pub account: String,
    pub away: bool,
    #[network(rename = "awayMessage")]
    pub away_message: String,
    #[network(rename = "idleTime")]
    pub idle_time: DateTime,
    #[network(rename = "loginTime")]
    pub login_time: DateTime,
    pub server: String,
    #[network(rename = "ircOperator")]
    pub irc_operator: String,
    #[network(rename = "lastAwayMessageTime")]
    pub last_away_message_time: DateTime,
    #[network(rename = "whoisServiceReply")]
    pub whois_service_reply: String,
    #[network(rename = "suserHost")]
    pub suser_host: String,
    pub encrypted: bool,
    pub channels: StringList,
    #[network(rename = "userModes")]
    pub user_modes: String,
}

#[cfg(test)]
mod tests {
    use crate::message::signalproxy::NetworkMap;
    use crate::primitive::{Variant, VariantMap};
    use time::OffsetDateTime;

    use super::*;

    fn get_runtime() -> IrcUser {
        IrcUser {
            user: s!("NickServ"),
            host: s!("services"),
            nick: s!("NickServ"),
            real_name: s!(""),
            account: s!(""),
            away: false,
            away_message: s!(""),
            idle_time: OffsetDateTime::unix_epoch(),
            login_time: OffsetDateTime::unix_epoch(),
            server: s!(""),
            irc_operator: s!(""),
            last_away_message_time: OffsetDateTime::unix_epoch(),
            whois_service_reply: s!(""),
            suser_host: s!(""),
            encrypted: false,
            channels: StringList::new(),
            user_modes: s!(""),
        }
    }

    fn get_network() -> VariantMap {
        map! {
            s!("suserHost") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("lastAwayMessageTime") => Variant::VariantList(vec!
                [
                    Variant::DateTime(
                        OffsetDateTime::unix_epoch() ,
                    ),
                ],
            ),
            s!("away") => Variant::VariantList(vec!
                [
                    Variant::bool(
                        false,
                    ),
                ],
            ),
            s!("ircOperator") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("account") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("loginTime") => Variant::VariantList(vec!
                [
                    Variant::DateTime(
                        OffsetDateTime::unix_epoch()
                    ),
                ],
            ),
            s!("userModes") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("host") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!("services"),
                    ),
                ],
            ),
            s!("whoisServiceReply") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("channels") => Variant::VariantList(vec!
                [
                    Variant::StringList(vec!
                        [],
                    ),
                ],
            ),
            s!("realName") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("nick") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!("NickServ"),
                    ),
                ],
            ),
            s!("idleTime") => Variant::VariantList(vec!
                [
                    Variant::DateTime(
                        OffsetDateTime::unix_epoch()
                    ),
                ],
            ),
            s!("encrypted") => Variant::VariantList(vec!
                [
                    Variant::bool(
                        false,
                    ),
                ],
            ),
            s!("awayMessage") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
            s!("user") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!("NickServ"),
                    ),
                ],
            ),
            s!("server") => Variant::VariantList(vec!
                [
                    Variant::String(
                        s!(""),
                    ),
                ],
            ),
        }
    }

    #[test]
    fn ircuser_to_network() {
        assert_eq!(get_runtime().to_network_map(), get_network())
    }

    #[test]
    fn ircuser_from_network() {
        assert_eq!(IrcUser::from_network_map(&mut get_network()), get_runtime())
    }

    #[test]
    fn vec_ircuser_to_network() {
        assert_eq!(get_runtime().to_network_map(), get_network())
    }
}
