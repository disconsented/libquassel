use std::{collections::HashMap, sync::Arc};

use druid::{
    widget::{Align, Either, Flex, Label, List, Split},
    AppDelegate, Command,
};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};
use widgets::{AliasManagerWidget, BufferViewWidget};

use libquassel::message::{objects::AliasManager, StatefulSyncableClient, SyncProxy};
use libquassel::message::{
    objects::{self, BufferViewManager},
    StatefulSyncableServer,
};

use tracing::debug;
use tracing_subscriber::prelude::*;

use crate::server::{Direction, Message, ServerWidget};

const SPACING: f64 = 10.0;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const WINDOW_TITLE: LocalizedString<StateTracker> = LocalizedString::new("StateTracker");

mod widgets;

mod command;
mod connect;
mod formatter;
mod server;

#[derive(Clone, Data, Lens)]
struct StateTracker {
    server: server::Server,
    messages: Arc<Vec<server::Message>>,
    alias_manager: Arc<AliasManager>,
    buffer_view_manager: Arc<BufferViewManager>,
    connected: bool,
    #[data(ignore)]
    syncer: Syncer,
}

impl StateTracker {
    fn new() -> StateTracker {
        let (sync_channel, rpc_channel) = SyncProxy::init(1024);

        StateTracker {
            server: server::Server::default(),
            messages: Arc::new(Vec::new()),
            alias_manager: Arc::new(AliasManager {
                aliases: Vec::new(),
            }),
            buffer_view_manager: Arc::new(BufferViewManager {
                buffer_view_configs: HashMap::new(),
            }),
            connected: false,
            syncer: Syncer {
                sync_channel,
                rpc_channel,
            },
        }
    }

    fn widget() -> impl Widget<StateTracker> {
        let either = Either::new(
            |server, _env| server.connected,
            Split::columns(
                Flex::column()
                    .with_child(Label::new("AliasManager"))
                    .with_child(AliasManagerWidget::new().lens(StateTracker::alias_manager))
                    .with_spacer(SPACING)
                    .with_child(Label::new("BufferViewManager"))
                    .with_child(BufferViewWidget::new().lens(StateTracker::buffer_view_manager)),
                List::new(|| {
                    Label::new(|item: &Message, _env: &_| format!("{:#?}", item)).padding(10.0)
                })
                .scroll()
                .vertical()
                .lens(StateTracker::messages),
            )
            .expand(),
            ServerWidget::new()
                .fix_width(200.0)
                .lens(StateTracker::server),
        );

        let layout = Flex::column()
            .with_flex_child(either, 1.0)
            .with_spacer(VERTICAL_WIDGET_SPACING);

        Align::centered(layout)
    }
}

impl Default for StateTracker {
    fn default() -> Self {
        Self::new()
    }
}

struct StateTrackerDelegate;
impl AppDelegate<StateTracker> for StateTrackerDelegate {
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &Command,
        data: &mut StateTracker,
        _env: &Env,
    ) -> druid::Handled {
        if let Some(_) = cmd.get(command::CONNECT) {
            debug!("got CONNECT command");

            data.connect(ctx.get_external_handle());
            data.connected = true;

            return druid::Handled::Yes;
        } else if let Some(msg) = cmd.get(command::ADD_MESSAGE) {
            let list = Arc::make_mut(&mut data.messages);
            list.push(msg.take().unwrap());
        } else if let Some(alias) = cmd.get(command::ALIASMANAGER_ADD_ALIAS) {
            let mut alias_manager = Arc::make_mut(&mut data.alias_manager).clone();
            alias_manager.add_alias(alias.take().unwrap());
            data.alias_manager = Arc::new(alias_manager);
        } else if let Some(initdata) = cmd.get(command::INITDATA) {
            let (_, initdata) = initdata.take().unwrap();
            match initdata.init_data {
                objects::Types::AliasManager(alias_manager) => {
                    data.alias_manager = Arc::new(alias_manager)
                }
                objects::Types::BufferViewManager(buffer_view_manager) => {
                    data.buffer_view_manager = Arc::new(buffer_view_manager)
                }
                objects::Types::BufferViewConfig(config) => {
                    let id: i32 = initdata.object_name.parse().unwrap();

                    let mut buffer_view_manager =
                        Arc::make_mut(&mut data.buffer_view_manager).clone();

                    buffer_view_manager.buffer_view_configs.insert(id, config);

                    data.buffer_view_manager = Arc::new(buffer_view_manager)
                }
                _ => (),
            }
        } else if let Some(msg) = cmd.get(command::SYNCMESSAGE) {
            let (direction, msg) = msg.take().unwrap();

            debug!("direction: {:#?}, msg: {:#?}", direction, msg);

            match msg.class_name.as_str() {
                "AliasManager" => {
                    let mut alias_manager = Arc::make_mut(&mut data.alias_manager).clone();

                    if direction == Direction::ServerToClient {
                        StatefulSyncableClient::sync(&mut alias_manager, msg);
                    } else {
                        StatefulSyncableServer::sync(&mut alias_manager, msg);
                    }

                    data.alias_manager = Arc::new(alias_manager);
                }
                "BufferViewConfig" => {
                    let mut buffer_view_manager =
                        Arc::make_mut(&mut data.buffer_view_manager).clone();

                    let id: i32 = msg.object_name.parse().unwrap();

                    let buffer_view_config = buffer_view_manager
                        .buffer_view_configs
                        .get_mut(&id)
                        .unwrap();

                    if direction == Direction::ServerToClient {
                        StatefulSyncableClient::sync(buffer_view_config, msg);
                    } else {
                        StatefulSyncableServer::sync(buffer_view_config, msg);
                    }

                    data.buffer_view_manager = Arc::new(buffer_view_manager);
                }

                _ => (),
            }
        }

        druid::Handled::No
    }
}

// impl Session for StateTracker {
//     fn alias_manager(&mut self) -> &mut AliasManager {
//         &mut Arc::make_mut(&mut self.alias_manager).clone()
//     }
// }

// TODO make this somehow deref or smth
#[derive(Clone)]
pub struct Syncer {
    sync_channel: crossbeam_channel::Receiver<libquassel::message::SyncMessage>,
    rpc_channel: crossbeam_channel::Receiver<libquassel::message::RpcCall>,
}

fn main() {
    // tracing_subscriber::fmt::fmt()
    //     .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
    //     .init();

    let filter = tracing_subscriber::filter::Targets::new()
        .with_default(tracing::Level::TRACE)
        .with_target("druid", tracing::metadata::LevelFilter::OFF);

    let env_filter = tracing_subscriber::EnvFilter::from_default_env();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE),
        )
        .with(filter)
        .with(env_filter)
        .init();

    // describe the main window
    let main_window = WindowDesc::new(StateTracker::widget())
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let initial_state = StateTracker::new();

    // start the application
    AppLauncher::with_window(main_window)
        .delegate(StateTrackerDelegate)
        .launch(initial_state)
        .expect("Failed to launch application");
}
