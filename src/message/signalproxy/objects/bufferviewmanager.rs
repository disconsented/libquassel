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
    pub buffer_view_configs: HashMap<i32, super::BufferViewConfig>,
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
