use std::collections::HashMap;

use crate::primitive::{DateTime, StringList, Variant, VariantMap};

#[allow(unused_imports)]
use crate::message::signalproxy::Network;
use libquassel_derive::Network;

impl Network for Vec<IrcUser> {
    type Item = VariantMap;

    fn to_network(&self) -> Self::Item {
        Variant::VariantMap(self.iter().fold(HashMap::new(), |mut res, v| {
            res.extend(v.to_network());

            res
        }))
    }
    fn from_network(input: &mut Self::Item) -> Self {
        todo!()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Network)]
#[network(repr = "maplist")]
pub struct IrcUser {
    user: String,
    host: String,
    nick: String,
    #[network(rename = "realName")]
    real_name: String,
    account: String,
    away: bool,
    #[network(rename = "awayMessage")]
    away_message: String,
    #[network(rename = "idleTime")]
    idle_time: DateTime,
    #[network(rename = "loginTime")]
    login_time: DateTime,
    server: String,
    #[network(rename = "ircOperator")]
    irc_operator: String,
    #[network(rename = "lastAwayMessageTime")]
    last_away_message_time: DateTime,
    #[network(rename = "whoisServiceReply")]
    whois_service_reply: String,
    #[network(rename = "suserHost")]
    suser_host: String,
    encrypted: bool,
    channels: StringList,
    #[network(rename = "userModes")]
    user_modes: String,
}

#[cfg(test)]
mod tests {
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
        assert_eq!(get_runtime().to_network(), get_network())
    }

    #[test]
    fn ircuser_from_network() {
        assert_eq!(IrcUser::from_network(&mut get_network()), get_runtime())
    }

    #[test]
    fn vec_ircuser_to_network() {
        assert_eq!(get_runtime().to_network(), get_network())
    }
}
