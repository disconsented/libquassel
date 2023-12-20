use std::collections::HashMap;

use crate::{
    message::{Syncable, Class},
    primitive::MessageType,
};

use libquassel_derive::{sync, NetworkList, NetworkMap};

#[derive(Default, Debug, Clone, PartialEq, NetworkList, NetworkMap)]
pub struct BufferSyncer {
    #[network(rename = "Activities", network = "list", variant = "VariantList")]
    pub activities: HashMap<i32, MessageType>,
    #[network(rename = "HighlightCounts", network = "list", variant = "VariantList")]
    pub highlight_counts: HashMap<i32, i32>,
    #[network(rename = "LastSeenMsg", network = "list", variant = "VariantList")]
    pub last_seen_msg: HashMap<i32, i64>,
    #[network(rename = "MarkerLines", network = "list", variant = "VariantList")]
    pub marker_line: HashMap<i32, i64>,
}

impl BufferSyncer {
    pub fn request_mark_buffer_as_read(&mut self, id: i32) {
        sync!("requestMarkBufferAsRead", [id]);
    }

    pub fn request_merge_buffers_permanently(&self, src_id: i32, target_id: i32) {
        sync!("requestMergeBuffersPermanently", [src_id, target_id]);
    }

    pub fn request_purge_buffer_ids(&self) {
        sync!("requestPurgeBufferIds", []);
    }

    pub fn request_remove_buffer(&self, id: i32) {
        sync!("requestRemoveBuffer", [id]);
    }

    pub fn request_rename_buffer(&self, id: i32) {
        sync!("requestRenameBuffer", [id]);
    }

    pub fn request_set_last_seen_msg(&self, id: i32, msgid: i32) {
        sync!("requestSetLastSeenMsg", [id, msgid]);
    }

    pub fn request_set_marker_line(&self, id: i32, msgid: i32) {
        sync!("requestSetMarkerLine", [id, msgid]);
    }

    // // S->C calls

    pub fn mark_buffer_as_read(&mut self, id: i32) {
        self.set_buffer_activity(id, MessageType::NONE);
        self.set_highlight_count(id, 0);

        #[cfg(feature = "server")]
        sync!("markBufferAsRead", [id]);
    }

    pub fn merge_buffers_permanently(&mut self, target: i32, source: i32) {
        if let Some(activities) = self.activities.remove(&source) {
            *self.activities.entry(target).or_insert(MessageType::NONE) |= activities;
        }

        if let Some(highlight_counts) = self.highlight_counts.remove(&source) {
            *self.highlight_counts.entry(target).or_default() += highlight_counts;
        }

        if let Some(last_seen_msg) = self.last_seen_msg.remove(&source) {
            let target = self.last_seen_msg.entry(target).or_default();
            if *target < last_seen_msg {
                *target = last_seen_msg
            };
        }

        if let Some(marker_line) = self.marker_line.remove(&source) {
            let target = self.marker_line.entry(target).or_default();
            if *target < marker_line {
                *target = marker_line
            };
        }

        #[cfg(feature = "server")]
        sync!("mergeBuffersPermanently", [source, target]);
    }

    // TODO remove buffer from bufferviews
    pub fn remove_buffer(&mut self, id: i32) {
        self.activities.remove(&id);
        self.highlight_counts.remove(&id);
        self.last_seen_msg.remove(&id);
        self.marker_line.remove(&id);

        #[cfg(feature = "server")]
        sync!("removeBuffer", [id]);
    }

    // TODO actually rename the buffer in whereever we should store buffers
    // and the BufferView
    #[allow(unused_variables)]
    pub fn rename_buffer(&mut self, id: i32, name: String) {
        #[cfg(feature = "server")]
        sync!("renameBuffer", [id, name]);
    }

    pub fn set_buffer_activity(&mut self, id: i32, activity: MessageType) {
        *self.activities.entry(id).or_insert(MessageType::NONE) = activity;

        #[cfg(feature = "server")]
        sync!("setBufferActivity", [id, activity.bits()]);
    }

    pub fn set_highlight_count(&mut self, id: i32, count: i32) {
        *self.highlight_counts.entry(id).or_default() = count;

        #[cfg(feature = "server")]
        sync!("setHighlightCount", [id, count]);
    }

    pub fn set_last_seen_msg(&mut self, id: i32, msg_id: i64) {
        *self.last_seen_msg.entry(id).or_default() = msg_id;

        #[cfg(feature = "server")]
        sync!("setHighlightCount", [id, msg_id]);
    }

    pub fn set_marker_line(&mut self, id: i32, msg_id: i64) {
        *self.marker_line.entry(id).or_default() = msg_id;

        #[cfg(feature = "server")]
        sync!("setHighlightCount", [id, msg_id]);
    }
}

#[cfg(feature = "client")]
impl crate::message::StatefulSyncableClient for BufferSyncer {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "markBufferAsRead" => self.mark_buffer_as_read(get_param!(msg)),
            "mergeBuffersPermanently" => {
                self.merge_buffers_permanently(get_param!(msg), get_param!(msg))
            }
            "removeBuffer" => self.remove_buffer(get_param!(msg)),
            "renameBuffer" => self.rename_buffer(get_param!(msg), get_param!(msg)),
            "setBufferActivity" => self.set_buffer_activity(
                get_param!(msg),
                MessageType::from_bits(get_param!(msg)).unwrap_or(MessageType::NONE),
            ),
            "setHighlightCount" => self.set_highlight_count(get_param!(msg), get_param!(msg)),
            "setLastSeenMsg" => self.set_last_seen_msg(get_param!(msg), get_param!(msg)),
            "setMarkerLine" => self.set_marker_line(get_param!(msg), get_param!(msg)),
            _ => (),
        }
    }
}

#[cfg(feature = "server")]
impl crate::message::StatefulSyncableServer for BufferSyncer {
    fn sync_custom(&mut self, mut msg: crate::message::SyncMessage)
    where
        Self: Sized,
    {
        match msg.slot_name.as_str() {
            "requestMarkBufferAsRead" => self.mark_buffer_as_read(get_param!(msg)),
            "requestMergeBuffersPermanently" => {
                self.merge_buffers_permanently(get_param!(msg), get_param!(msg))
            }
            "requestPurgeBufferIds" => (),
            "requestRemoveBuffer" => self.remove_buffer(get_param!(msg)),
            "requestRenameBuffer" => self.rename_buffer(get_param!(msg), get_param!(msg)),
            "requestSetLastSeenMsg" => self.set_last_seen_msg(get_param!(msg), get_param!(msg)),
            "requestSetMarkerLine" => self.set_marker_line(get_param!(msg), get_param!(msg)),
            _ => (),
        }
    }
}

impl Syncable for BufferSyncer {
    const CLASS: Class = Class::BufferSyncer;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::signalproxy::translation::NetworkList;
    use crate::primitive::{Variant, VariantList};
    use pretty_assertions::assert_eq;

    fn get_network() -> VariantList {
        vec![
            Variant::ByteArray(s!("Activities")),
            Variant::VariantList(vec![
                Variant::i32(1),
                Variant::i32(0),
                Variant::i32(2),
                Variant::i32(0),
                Variant::i32(3),
                Variant::i32(0),
                Variant::i32(4),
                Variant::i32(0),
                Variant::i32(5),
                Variant::i32(0),
            ]),
            Variant::ByteArray(s!("HighlightCounts")),
            Variant::VariantList(vec![
                Variant::i32(1),
                Variant::i32(0),
                Variant::i32(2),
                Variant::i32(0),
                Variant::i32(3),
                Variant::i32(0),
                Variant::i32(4),
                Variant::i32(0),
                Variant::i32(5),
                Variant::i32(0),
            ]),
            Variant::ByteArray(s!("LastSeenMsg")),
            Variant::VariantList(vec![
                Variant::i32(1),
                Variant::i64(2185),
                Variant::i32(2),
                Variant::i64(2188),
                Variant::i32(3),
                Variant::i64(860),
                Variant::i32(4),
                Variant::i64(2183),
                Variant::i32(5),
                Variant::i64(2180),
            ]),
            Variant::ByteArray(s!("MarkerLines")),
            Variant::VariantList(vec![
                Variant::i32(1),
                Variant::i64(2185),
                Variant::i32(2),
                Variant::i64(2188),
                Variant::i32(3),
                Variant::i64(860),
                Variant::i32(4),
                Variant::i64(1527),
                Variant::i32(5),
                Variant::i64(2180),
            ]),
        ]
    }

    fn get_runtime() -> BufferSyncer {
        BufferSyncer {
            activities: map! {
                1 => MessageType::NONE,
                2 => MessageType::NONE,
                3 => MessageType::NONE,
                4 => MessageType::NONE,
                5 => MessageType::NONE,
            },
            highlight_counts: map! {
                1 => 0,
                2 => 0,
                3 => 0,
                4 => 0,
                5 => 0,
            },
            last_seen_msg: map! {
                1 => 2185,
                2 => 2188,
                3 => 860,
                4 => 2183,
                5 => 2180,
            },
            marker_line: map! {
                1 => 2185,
                2 => 2188,
                3 => 860,
                4 => 1527,
                5 => 2180,
            },
        }
    }

    // Disabled cus not sorted
    // #[test]
    // fn buffersyncer_to_network() {
    //     assert_eq!(get_runtime().to_network(), get_network())
    // }

    #[test]
    fn buffersyncer_from_network() {
        assert_eq!(
            BufferSyncer::from_network_list(&mut get_network()),
            get_runtime()
        )
    }
}
