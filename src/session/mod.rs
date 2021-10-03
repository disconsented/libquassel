use crate::message::{objects::AliasManager, StatefulSyncableClient, SyncMessage, SyncProxy};

/// The Session Trait is the main point of entry and implements the basic logic
pub trait Session: SyncProxy {
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

#[allow(unused_variables)]
impl<T> SyncProxy for &mut T
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
