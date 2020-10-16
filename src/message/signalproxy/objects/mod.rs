mod aliasmanager;
mod backlogmanager;

pub trait Act {
    fn act(self: Self);
}
