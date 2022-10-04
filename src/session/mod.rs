use crate::message::{objects::{*, Types}, InitData, StatefulSyncableClient, SyncMessage, Syncable, Class};

// TODO implement nested types init and sync like BufferViewConfig in BufferViewManager

#[derive(Default, Debug)]
pub struct Session {
    pub alias_manager: AliasManager,
    pub buffer_syncer: BufferSyncer,
    pub backlog_manager: BacklogManager,
    pub buffer_view_manager: BufferViewManager,
    pub cert_manager: CertManager,
    pub core_info: CoreInfo,
    pub highlight_rule_manager: HighlightRuleManager,
    pub identity: Identity,
    pub ignore_list_manager: IgnoreListManager,
}

/// The Session Trait is the main point of entry and implements the basic logic
pub trait SessionManager {
    fn alias_manager(&mut self) -> &mut AliasManager;
    fn buffer_syncer(&mut self) -> &mut BufferSyncer;
    fn backlog_manager(&mut self) -> &mut BacklogManager;
    fn buffer_view_manager(&mut self) -> &mut BufferViewManager;
    fn cert_manager(&mut self) -> &mut CertManager;
    fn core_info(&mut self) -> &mut CoreInfo;
    fn highlight_rule_manager(&mut self) -> &mut HighlightRuleManager;
    fn identity(&mut self) -> &mut Identity;
    fn ignore_list_manager(&mut self) -> &mut IgnoreListManager;

    fn sync(&mut self, msg: SyncMessage)
    where
        Self: Sized,
    {
        match msg.class_name {
            Class::AliasManager => self.alias_manager().sync(msg),
            Class::BufferSyncer => self.buffer_syncer().sync(msg),
            Class::BufferViewConfig => (),
            Class::BufferViewManager => self.buffer_view_manager().sync(msg),
            Class::CoreInfo => self.core_info().sync(msg),
            Class::CoreData => (),
            Class::HighlightRuleManager => self.highlight_rule_manager().sync(msg),
            Class::Identity => self.identity().sync(msg),
            Class::IgnoreListManager => self.ignore_list_manager().sync(msg),
            Class::CertManager => self.cert_manager().sync(msg),
            Class::Network => (),
            Class::NetworkInfo => (),
            Class::NetworkConfig => (),
            Class::IrcChannel => (),
            Class::IrcUser => (),
            Class::Unknown => (),
        }
    }

    fn init(&mut self, data: InitData) {
        match data.init_data {
            Types::AliasManager(data) => {self.alias_manager().init(data)}
            Types::BufferSyncer(data) => self.buffer_syncer().init(data),
            Types::BufferViewConfig(_) => (),
            Types::BufferViewManager(data) => self.buffer_view_manager().init(data),
            Types::CoreData(data) => self.core_info().set_core_data(data),
            Types::HighlightRuleManager(data) => self.highlight_rule_manager().init(data),
            Types::IgnoreListManager(data) => self.ignore_list_manager().init(data),
            Types::CertManager(data) => self.cert_manager().init(data),
            Types::Network(_) => (),
            Types::NetworkInfo(_) => (),
            Types::NetworkConfig(_) => (),
            Types::Unknown(_) => (),
        }
    }
}

impl SessionManager for Session {
    fn alias_manager(&mut self) -> &mut AliasManager {
        &mut self.alias_manager
    }

    fn buffer_syncer(&mut self) -> &mut BufferSyncer {
        &mut self.buffer_syncer
    }

    fn backlog_manager(&mut self) -> &mut BacklogManager {
        &mut self.backlog_manager
    }

    fn buffer_view_manager(&mut self) -> &mut BufferViewManager {
        &mut self.buffer_view_manager
    }

    fn cert_manager(&mut self) -> &mut CertManager {
        &mut self.cert_manager
    }

    fn core_info(&mut self) -> &mut CoreInfo {
        &mut self.core_info
    }

    fn highlight_rule_manager(&mut self) -> &mut HighlightRuleManager {
        &mut self.highlight_rule_manager
    }

    fn identity(&mut self) -> &mut Identity {
        &mut self.identity
    }

    fn ignore_list_manager(&mut self) -> &mut IgnoreListManager {
        &mut self.ignore_list_manager
    }
}
