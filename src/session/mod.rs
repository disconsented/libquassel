use crate::message::{objects::AliasManager, SyncMessage, SyncProxy};

/// The Session Trait is the main point of entry and implements the basic logic
pub trait Session: SyncProxy {
    fn alias_manager(&self) -> AliasManager;

    fn handle_syncmessage(&self, msg: SyncMessage)
    where
        Self: Sized,
    {
        match msg.class_name.as_str() {
            "AliasManager" => self.alias_manager().handle_syncmessage(self, msg),
            _ => (),
        }
    }
}

impl<T> Session for &T
where
    T: Session,
{
    fn alias_manager(&self) -> AliasManager {
        todo!()
    }
}

#[allow(unused_variables)]
impl<T> SyncProxy for &T
where
    T: SyncProxy,
{
    fn sync(
        &self,
        class_name: &str,
        object_name: Option<&str>,
        function: &str,
        params: crate::primitive::VariantList,
    ) {
        todo!()
    }

    fn rpc(&self, function: &str, params: crate::primitive::VariantList) {
        todo!()
    }
}
