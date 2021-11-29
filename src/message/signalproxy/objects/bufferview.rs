use std::{collections::HashMap, convert::TryFrom, convert::TryInto};

use libquassel_derive::{NetworkList, NetworkMap};

use crate::message::signalproxy::translation::Network;
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
