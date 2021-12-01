use crate::message::objects::AliasManager;

/// The Session Trait is the main point of entry and implements the basic logic
pub trait Session {
    fn alias_manager(&mut self) -> &mut AliasManager;

    // fn sync(&mut self, msg: SyncMessage)
    // where
    //     Self: Sized,
    // {
    //     match msg.class_name.as_str() {
    //         "AliasManager" => self.alias_manager().sync(self, msg),
    //         _ => (),
    //     }
    // }
}

impl<T> Session for &T
where
    T: Session,
{
    fn alias_manager(&mut self) -> &mut AliasManager {
        todo!()
    }
}
