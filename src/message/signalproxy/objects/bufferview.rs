use std::{collections::HashMap, convert::TryFrom, convert::TryInto};

use libquassel_derive::{NetworkList, NetworkMap};

use crate::message::signalproxy::translation::Network;
use crate::message::{StatefulSyncableClient, StatefulSyncableServer, SyncProxy, Syncable};
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

#[derive(Debug, Clone, PartialEq, NetworkList, NetworkMap)]
pub struct BufferViewConfig {
    #[network(rename = "BufferList", network, variant = "VariantList")]
    pub buffers: Vec<i32>,
    #[network(rename = "RemovedBuffers", network, variant = "VariantList")]
    pub removed_buffers: Vec<i32>,
    #[network(rename = "TemporarilyRemovedBuffers", network, variant = "VariantList")]
    pub temporarily_removed_buffers: Vec<i32>,

    // TODO think about how to handle the buffer view id
    //   we might introduce a default flag for the network macro
    // #[network(rename = "bufferViewId")]
    // pub buffer_view_id: i32,
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
    #[network(rename = "allowedBufferTypes")]
    pub allowed_buffers_types: i32,
    #[network(rename = "minimumActivity")]
    pub minimum_activity: i32,
    #[network(rename = "showSearch")]
    pub show_search: bool,
}

#[allow(dead_code)]
impl BufferViewConfig {
    // TODO add sync to all functions
    // TODO requestAddBuffer(bufferId: BufferId, pos: Int)
    // TODO requestMoveBuffer(bufferId: BufferId, pos: Int)
    // TODO requestRemoveBuffer(bufferId: BufferId)
    // TODO requestRemoveBufferPermanently(bufferId: BufferId)
    // TODO requestSetBufferViewName(bufferViewName: QString)
    // /**
    //  * Replaces all properties of the object with the content of the
    //  * "properties" parameter. This parameter is in network representation.
    //  */
    // DONE requestUpdate(properties: QVariantMap)

    // // S->C calls
    // DONE addBuffer(bufferId: BufferId, pos: Int)
    // DONE moveBuffer(bufferId: BufferId, pos: Int)
    // DONE removeBuffer(bufferId: BufferId)
    // DONE removeBufferPermanently(bufferId: BufferId)
    // TODO setAddNewBuffersAutomatically(addNewBuffersAutomatically: Bool)
    // TODO setAllowedBufferTypes(bufferTypes: Int)
    // TODO setBufferViewName(bufferViewName: QString)
    // TODO setDisableDecoration(disableDecoration: Bool)
    // TODO setHideInactiveBuffers(hideInactiveBuffers: Bool)
    // TODO setHideInactiveNetworks(hideInactiveNetworks: Bool)
    // TODO setMinimumActivity(activity: Int)
    // TODO setNetworkId(networkId: NetworkId)
    // TODO setShowSearch(showSearch: Bool)
    // TODO setSortAlphabetically(sortAlphabetically: Bool)

    fn add_buffer(&mut self, id: i32, pos: usize) {
        if !self.buffers.contains(&id) {
            self.buffers.insert(pos, id)
        }

        #[cfg(feature = "server")]
        {
            // TODO replace the None with self.buffer_view_id
            self.send_sync(None, "addBuffer", vec![id.into(), (pos as i32).into()])
        }
    }

    fn move_buffer(&mut self, id: i32, pos: usize) {
        let old_pos = self.buffers.iter().position(|&x| x == id).unwrap();
        self.buffers.remove(old_pos);
        self.buffers.insert(pos, id);
    }

    fn remove_buffer(&mut self, id: i32) {
        if self.buffers.contains(&id) {
            let old_pos = self.buffers.iter().position(|&x| x == id).unwrap();
            self.buffers.remove(old_pos);
        }

        if self.removed_buffers.contains(&id) {
            let old_pos = self.removed_buffers.iter().position(|&x| x == id).unwrap();
            self.removed_buffers.remove(old_pos);
        }

        if !self.temporarily_removed_buffers.contains(&id) {
            self.buffers.push(id)
        }
    }

    fn remove_buffer_permanently(&mut self, id: i32) {
        if self.buffers.contains(&id) {
            let old_pos = self.buffers.iter().position(|&x| x == id).unwrap();
            self.buffers.remove(old_pos);
        }

        if self.temporarily_removed_buffers.contains(&id) {
            let old_pos = self.removed_buffers.iter().position(|&x| x == id).unwrap();
            self.removed_buffers.remove(old_pos);
        }

        if !self.removed_buffers.contains(&id) {
            self.buffers.push(id)
        }
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
            _ => (),
        }
    }
}

impl Syncable for BufferViewConfig {
    const CLASS: &'static str = "BufferViewConfig";
}
