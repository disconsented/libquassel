use druid::{Selector, SingleUse};
use libquassel::message::{objects::Alias, InitData, SyncMessage};

use crate::server::Direction;

pub const CONNECT: Selector = Selector::new("connect");
pub const ADD_MESSAGE: Selector<SingleUse<crate::Message>> = Selector::new("add_message");

pub const ALIASMANAGER_ADD_ALIAS: Selector<SingleUse<Alias>> =
    Selector::new("aliasmanager_add_alias");

pub const SYNCMESSAGE: Selector<SingleUse<(Direction, SyncMessage)>> = Selector::new("syncmessage");
pub const INITDATA: Selector<SingleUse<(Direction, InitData)>> = Selector::new("initdata");
