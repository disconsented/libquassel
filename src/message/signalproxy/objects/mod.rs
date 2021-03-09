mod aliasmanager;
mod buffersyncer;
mod identity;
// mod ircchannel;
// mod ircuser;
// mod network;
// mod networkinfo;

pub use aliasmanager::*;
pub use buffersyncer::*;
pub use identity::*;

pub trait Act {
    fn act(self: Self);
}
