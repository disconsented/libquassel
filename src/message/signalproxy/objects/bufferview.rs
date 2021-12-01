use std::{collections::HashMap, convert::TryFrom, convert::TryInto};

use libquassel_derive::sync;
use libquassel_derive::{NetworkList, NetworkMap};

use crate::message::signalproxy::translation::Network;

#[allow(unused_imports)]
use crate::message::StatefulSyncableClient;
#[allow(unused_imports)]
use crate::message::StatefulSyncableServer;
use crate::message::Syncable;

use crate::primitive::{Variant, VariantList};

#[derive(Debug, Clone, PartialEq)]
// #[network(repr = "list")]
pub struct BufferViewManager {
    // #[network(rename = "bufferViewConfigs", network, variant = "VariantMap")]
    pub buffer_view_configs: HashMap<i32, BufferViewConfig>,
    // // C->S calls

    // requestCreateBufferView(properties: QVariantMap)
    // requestCreateBufferViews(properties: QVariantList)
    // requestDeleteBufferView(bufferViewId: Int)
    // requestDeleteBufferViews(bufferViews: QVariantList)

    // // S->C calls

    // addBufferViewConfig(bufferViewConfigId: Int)
    // deleteBufferViewConfig(bufferViewConfigId: Int)
    // newBufferViewConfig(bufferViewConfigId: Int)
    // /**
    //  * Replaces all properties of the object with the content of the
    //  * "properties" parameter. This parameter is in network representation.
    //  */
    // update(properties: QVariantMap)
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

#[derive(Debug, Default, Clone, PartialEq, NetworkList, NetworkMap)]
pub struct BufferViewConfig {
    #[network(rename = "BufferList", network, variant = "VariantList")]
    pub buffers: Vec<i32>,
    #[network(rename = "RemovedBuffers", network, variant = "VariantList")]
    pub removed_buffers: Vec<i32>,
    #[network(rename = "TemporarilyRemovedBuffers", network, variant = "VariantList")]
    pub temporarily_removed_buffers: Vec<i32>,

    #[network(rename = "bufferViewId", default, skip)]
    pub buffer_view_id: i32,
    #[network(rename = "bufferViewName")]
    pub buffer_view_name: String,
    #[network(rename = "networkId")]
    pub network_id: i32,
    #[network(rename = "addNewBuffersAutomatically")]
    pub add_new_buffers_automatically: bool,
    #[network(rename = "sortAlphabetically")]
    pub sort_alphabetically: bool,
    #[network(rename = "hideInactiveBuffers")]
    pub hide_inactive_buffers: bool,
    #[network(rename = "hideInactiveNetworks")]
    pub hide_inactive_networks: bool,
    #[network(rename = "disableDecoration")]
    pub disable_decoration: bool,
    // TODO use bitflags for buffertypes
    #[network(rename = "allowedBufferTypes")]
    pub allowed_buffer_types: i32,
    #[network(rename = "minimumActivity")]
    pub minimum_activity: i32,
    #[network(rename = "showSearch")]
    pub show_search: bool,
}

#[allow(dead_code)]
impl BufferViewConfig {
    pub fn request_add_buffer(&self, id: i32, pos: usize) {
        sync!("requestAddBuffer", [id, (pos as i32)]);
    }

    pub fn add_buffer(&mut self, id: i32, pos: usize) {
        if !self.buffers.contains(&id) {
            self.buffers.insert(pos, id)
        }

        if let Some(old_pos) = self.removed_buffers.iter().position(|&x| x == id) {
            self.removed_buffers.remove(old_pos);
        }

        if let Some(old_pos) = self
            .temporarily_removed_buffers
            .iter()
            .position(|&x| x == id)
        {
            self.temporarily_removed_buffers.remove(old_pos);
        }

        #[cfg(feature = "server")]
        sync!("addBuffer", [id, (pos as i32)]);
    }

    pub fn request_move_buffer(&self, id: i32, pos: usize) {
        sync!("requestMoveBuffer", [id, (pos as i32)]);
    }

    pub fn move_buffer(&mut self, id: i32, pos: usize) {
        let old_pos = self.buffers.iter().position(|&x| x == id).unwrap();
        self.buffers.remove(old_pos);
        self.buffers.insert(pos, id);

        #[cfg(feature = "server")]
        sync!("moveBuffer", [id, (pos as i32)]);
    }

    pub fn request_remove_buffer(&mut self, id: i32) {
        sync!("requestRemoveBuffer", [id]);
    }

    pub fn remove_buffer(&mut self, id: i32) {
        if let Some(old_pos) = self.buffers.iter().position(|&x| x == id) {
            self.buffers.remove(old_pos);
        }

        if let Some(old_pos) = self.removed_buffers.iter().position(|&x| x == id) {
            self.removed_buffers.remove(old_pos);
        }

        if !self.temporarily_removed_buffers.contains(&id) {
            self.temporarily_removed_buffers.push(id)
        }

        #[cfg(feature = "server")]
        sync!("removeBuffer", [id]);
    }

    pub fn request_remove_buffer_permanently(&mut self, id: i32) {
        sync!("requestRemoveBufferPermanently", [id]);
    }

    pub fn remove_buffer_permanently(&mut self, id: i32) {
        if let Some(old_pos) = self.buffers.iter().position(|&x| x == id) {
            self.buffers.remove(old_pos);
        }

        if let Some(old_pos) = self
            .temporarily_removed_buffers
            .iter()
            .position(|&x| x == id)
        {
            self.temporarily_removed_buffers.remove(old_pos);
        }

        if !self.removed_buffers.contains(&id) {
            self.removed_buffers.push(id)
        }

        #[cfg(feature = "server")]
        sync!("removeBufferPermanently", [id]);
    }
}

#[cfg(feature = "client")]
impl StatefulSyncableClient for BufferViewConfig {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        log::debug!("entering bufferviewconfig sync_custom()");
        match msg.slot_name.as_str() {
            "addBuffer" => self.add_buffer(
                msg.params.remove(0).try_into().unwrap(),
                i32::try_from(msg.params.remove(0)).unwrap() as usize,
            ),
            "moveBuffer" => self.move_buffer(
                msg.params.remove(0).try_into().unwrap(),
                i32::try_from(msg.params.remove(0)).unwrap() as usize,
            ),
            "removeBuffer" => self.remove_buffer(msg.params.remove(0).try_into().unwrap()),
            "removeBufferPermanently" => {
                self.remove_buffer_permanently(msg.params.remove(0).try_into().unwrap())
            }
            _ => (),
        }
    }
}

#[cfg(feature = "server")]
impl StatefulSyncableServer for BufferViewConfig {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "requestAddBuffer" => self.add_buffer(
                msg.params.remove(0).try_into().unwrap(),
                i32::try_from(msg.params.remove(0)).unwrap() as usize,
            ),
            "requestMoveBuffer" => self.move_buffer(
                msg.params.remove(0).try_into().unwrap(),
                i32::try_from(msg.params.remove(0)).unwrap() as usize,
            ),
            "requestRemoveBuffer" => self.remove_buffer(msg.params.remove(0).try_into().unwrap()),
            "requestRemoveBufferPermanently" => {
                self.remove_buffer_permanently(msg.params.remove(0).try_into().unwrap())
            }
            "setAddNewBuffersAutomatically" => {
                self.add_new_buffers_automatically = msg.params.remove(0).try_into().unwrap()
            }
            "setAllowedBufferTypes" => {
                self.allowed_buffer_types = msg.params.remove(0).try_into().unwrap()
            }
            "setBufferViewName" => self.buffer_view_name = msg.params.remove(0).try_into().unwrap(),
            "setDisableDecoration" => {
                self.disable_decoration = msg.params.remove(0).try_into().unwrap()
            }
            "setHideInactiveBuffers" => {
                self.hide_inactive_buffers = msg.params.remove(0).try_into().unwrap()
            }
            "setHideInactiveNetworks" => {
                self.hide_inactive_networks = msg.params.remove(0).try_into().unwrap()
            }
            "setMinimumActivity" => {
                self.minimum_activity = msg.params.remove(0).try_into().unwrap()
            }
            "setNetworkId" => self.network_id = msg.params.remove(0).try_into().unwrap(),
            "setShowSearch" => self.show_search = msg.params.remove(0).try_into().unwrap(),
            "setSortAlphabetically" => {
                self.sort_alphabetically = msg.params.remove(0).try_into().unwrap()
            }
            _ => (),
        }
    }
}

impl Syncable for BufferViewConfig {
    const CLASS: &'static str = "BufferViewConfig";

    fn send_sync(&self, function: &str, params: VariantList) {
        crate::message::signalproxy::SYNC_PROXY.get().unwrap().sync(
            Self::CLASS,
            Some(&self.buffer_view_id.to_string()),
            function,
            params,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bufferviewconfig_sample() -> BufferViewConfig {
        BufferViewConfig {
            buffers: vec![1, 2, 3],
            removed_buffers: vec![4, 5],
            temporarily_removed_buffers: vec![6, 7],
            ..Default::default()
        }
    }

    #[test]
    fn bufferviewconfig_add_buffer() {
        // Add existing buffer, no change
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.add_buffer(1, 2);
        assert_eq!(bufferviewconfig_sample(), buffer_view_config);

        // Add new buffer
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.add_buffer(10, 1);
        assert_eq!(
            BufferViewConfig {
                buffers: vec![1, 10, 2, 3],
                removed_buffers: vec![4, 5],
                temporarily_removed_buffers: vec![6, 7],
                ..Default::default()
            },
            buffer_view_config
        );

        // Add new buffer, remove from removed buffers
        let mut buffer_view_config = BufferViewConfig {
            buffers: vec![1, 2, 3],
            removed_buffers: vec![4, 5, 10],
            temporarily_removed_buffers: vec![6, 7, 10],
            ..Default::default()
        };
        buffer_view_config.add_buffer(10, 1);
        assert_eq!(
            BufferViewConfig {
                buffers: vec![1, 10, 2, 3],
                removed_buffers: vec![4, 5],
                temporarily_removed_buffers: vec![6, 7],
                ..Default::default()
            },
            buffer_view_config
        );
    }

    #[test]
    fn bufferviewconfig_remove_buffer() {
        // Remove already removed buffer
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.remove_buffer(6);
        assert_eq!(bufferviewconfig_sample(), buffer_view_config);

        // Remove buffer
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.remove_buffer(1);
        assert_eq!(
            BufferViewConfig {
                buffers: vec![2, 3],
                removed_buffers: vec![4, 5],
                temporarily_removed_buffers: vec![6, 7, 1],
                ..Default::default()
            },
            buffer_view_config
        );
    }

    #[test]
    fn bufferviewconfig_remove_buffer_permanently() {
        // Remove already removed buffer
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.remove_buffer_permanently(4);
        assert_eq!(bufferviewconfig_sample(), buffer_view_config);

        // Remove buffer
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.remove_buffer_permanently(1);
        assert_eq!(
            BufferViewConfig {
                buffers: vec![2, 3],
                removed_buffers: vec![4, 5, 1],
                temporarily_removed_buffers: vec![6, 7],
                ..Default::default()
            },
            buffer_view_config
        );
    }

    #[test]
    fn bufferviewconfig_move_buffer() {
        // Do nothing
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.move_buffer(1, 0);
        assert_eq!(bufferviewconfig_sample(), buffer_view_config);

        // Move buffer
        let mut buffer_view_config = bufferviewconfig_sample();
        buffer_view_config.move_buffer(1, 1);
        assert_eq!(
            BufferViewConfig {
                buffers: vec![2, 1, 3],
                removed_buffers: vec![4, 5],
                temporarily_removed_buffers: vec![6, 7],
                ..Default::default()
            },
            buffer_view_config
        );
    }
}
