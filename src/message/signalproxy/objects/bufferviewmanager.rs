use std::collections::HashMap;

use libquassel_derive::sync;

#[allow(unused_imports)]
use crate::message::StatefulSyncableClient;
#[allow(unused_imports)]
use crate::message::StatefulSyncableServer;
use crate::message::{NetworkMap, Syncable};

use crate::primitive::{Variant, VariantList, VariantMap};

use super::BufferViewConfig;

#[derive(Debug, Clone, PartialEq)]
pub struct BufferViewManager {
    pub buffer_view_configs: HashMap<i32, BufferViewConfig>,
}

// TODO initialize the BufferViewConfigs from somewhere
// TODO add buffer view configs, where does the data come from?
impl BufferViewManager {
    pub fn request_create_buffer_view(&self, properties: BufferViewConfig) {
        sync!("requestCreateBufferView", [properties.to_network_map()])
    }

    pub fn request_create_buffer_views(&self, properties: &[BufferViewConfig]) {
        self.send_sync(
            "requestCreateBufferViews",
            properties
                .iter()
                .map(|view| view.to_network_map().into())
                .collect(),
        )
    }

    pub fn request_delete_buffer_view(&self, id: i32) {
        sync!("requestDeleteBufferView", [id])
    }

    pub fn request_delete_buffer_views(&self, ids: &[i32]) {
        self.send_sync(
            "requestCreateBufferViews",
            ids.iter().map(|id| (*id).into()).collect(),
        )
    }

    #[cfg(feature = "client")]
    #[allow(unused_variables)]
    pub fn add_buffer_view_config(&mut self, id: i32) {
        // TODO init!("BufferViewConfig", id);
    }

    #[cfg(feature = "server")]
    pub fn add_buffer_view_config(&mut self, config: BufferViewConfig) {
        self.buffer_view_configs.insert(0, config);

        sync!("addBufferViewConfig", [0]);
    }

    pub fn delete_buffer_view_config(&mut self, id: i32) {
        if self.buffer_view_configs.contains_key(&id) {
            self.buffer_view_configs.remove(&id);
        }

        #[cfg(feature = "server")]
        sync!("deleteBufferViewConfig", [id])
    }
}

#[cfg(feature = "client")]
impl StatefulSyncableClient for BufferViewManager {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "addBufferViewConfig" | "newBufferViewConfig" => {
                self.add_buffer_view_config(msg.params.remove(0).try_into().unwrap())
            }
            "deleteBufferViewConfig" => {
                self.delete_buffer_view_config(msg.params.remove(0).try_into().unwrap())
            }
            _ => (),
        }
    }
}

#[cfg(feature = "server")]
impl StatefulSyncableServer for BufferViewManager {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "requestCreateBufferView" => self.add_buffer_view_config(
                BufferViewConfig::from_network_map(&mut msg.params.remove(0).try_into().unwrap()),
            ),
            "requestCreateBufferViews" => {
                let views: VariantList = msg.params.remove(0).try_into().unwrap();
                views.into_iter().for_each(|view| {
                    self.add_buffer_view_config(BufferViewConfig::from_network_map(
                        &mut view.try_into().unwrap(),
                    ))
                });
            }
            "requestDeleteBufferView" => {
                self.delete_buffer_view_config(msg.params.remove(0).try_into().unwrap())
            }
            "requestDeleteBufferViews" => {
                let ids: VariantList = msg.params.remove(0).try_into().unwrap();
                ids.into_iter()
                    .for_each(|id| self.delete_buffer_view_config(id.try_into().unwrap()));
            }
            _ => (),
        }
    }
}

impl Syncable for BufferViewManager {
    const CLASS: &'static str = "BufferViewManager";
}

impl super::NetworkList for BufferViewManager {
    fn to_network_list(&self) -> VariantList {
        let mut res = Vec::with_capacity(2);

        res.push(Variant::ByteArray(s!("bufferViewIds")));
        res.push(Variant::VariantList(
            self.buffer_view_configs
                .iter()
                .map(|(k, _)| i32::try_into(*k).unwrap())
                .collect(),
        ));

        return res;
    }

    fn from_network_list(_input: &mut VariantList) -> Self {
        // TODO Somehow do the initrequests for all the IDs we get here
        Self {
            buffer_view_configs: HashMap::new(),
        }
    }
}

impl super::NetworkMap for BufferViewManager {
    type Item = VariantMap;

    fn to_network_map(&self) -> Self::Item {
        let mut res = VariantMap::new();

        res.insert(
            s!("bufferViewIds"),
            Variant::VariantList(
                self.buffer_view_configs
                    .iter()
                    .map(|(k, _)| i32::try_into(*k).unwrap())
                    .collect(),
            ),
        );

        return res;
    }

    fn from_network_map(_input: &mut Self::Item) -> Self {
        // TODO Somehow do the initrequests for all the IDs we get here
        Self {
            buffer_view_configs: HashMap::new(),
        }
    }
}
