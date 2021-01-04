// mod aliasmanager;
// mod backlogmanager;

mod buffersyncer;
mod identity;

pub use buffersyncer::*;
pub use identity::*;

pub trait Act {
    fn act(self: Self);
}
