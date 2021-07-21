use std::collections::HashMap;

use crate::primitive::MessageType;
use libquassel_derive::Network;

#[derive(Debug, Clone, PartialEq, Network)]
#[network(repr = "list")]
pub struct BufferSyncer {
    #[network(rename = "Activities", network, variant = "VariantList")]
    pub activities: HashMap<i32, MessageType>,
    #[network(rename = "HighlightCounts", network, variant = "VariantList")]
    pub highlight_counts: HashMap<i32, i32>,
    #[network(rename = "LastSeenMsg", network, variant = "VariantList")]
    pub last_seen_msg: HashMap<i32, i64>,
    #[network(rename = "MarkerLines", network, variant = "VariantList")]
    pub marker_line: HashMap<i32, i64>,
}

pub trait BufferSyncerServer {
    fn activities(self: &Self) -> &HashMap<u32, MessageType>;
    fn activities_mut(self: &mut Self) -> &mut HashMap<u32, MessageType>;

    fn highlight_counts(self: &Self) -> &HashMap<u32, u32>;
    fn highlight_counts_mut(self: &mut Self) -> &mut HashMap<u32, u32>;

    fn last_seen_msg(self: &Self) -> &HashMap<u32, u32>;
    fn last_seen_msg_mut(self: &mut Self) -> &mut HashMap<u32, u32>;

    fn marker_line(self: &Self) -> &HashMap<u32, u32>;
    fn marker_line_mut(self: &mut Self) -> &mut HashMap<u32, u32>;

    fn request_mark_buffer_as_read(buffer: u32);
    fn request_merge_buffers_permanently(buffer1: u32, buffer2: u32);
    fn request_purge_buffer_ids();
    fn request_remove_buffer(buffer: u32);
    fn request_rename_buffer(buffer: u32, new_name: String);

    fn request_set_last_seen_msg(self: &mut Self, buffer: u32, msgid: u32) {
        self.last_seen_msg_mut().insert(buffer, msgid);
    }

    fn request_set_marker_line(self: &mut Self, buffer: u32, msgid: u32) {
        self.marker_line_mut().insert(buffer, msgid);
    }
}

pub trait BufferSyncerClient {
    fn activities(self: &Self) -> &HashMap<u32, MessageType>;
    fn activities_mut(self: &mut Self) -> &mut HashMap<u32, MessageType>;

    fn highlight_counts(self: &Self) -> &HashMap<u32, u32>;
    fn highlight_counts_mut(self: &mut Self) -> &mut HashMap<u32, u32>;

    fn last_seen_msg(self: &Self) -> &HashMap<u32, u32>;
    fn last_seen_msg_mut(self: &mut Self) -> &mut HashMap<u32, u32>;

    fn marker_line(self: &Self) -> &HashMap<u32, u32>;
    fn marker_line_mut(self: &mut Self) -> &mut HashMap<u32, u32>;

    fn mark_buffer_as_read(buffer: u32);
    fn merge_buffers_permanently(buffer1: u32, buffer2: u32);
    fn remove_buffer(buffer: u32);
    fn rename_buffer(buffer: u32, new_name: String);

    fn set_buffer_activity(self: &mut Self, buffer: u32, activity: MessageType) {
        self.activities_mut().insert(buffer, activity);
    }

    fn set_highlight_count(self: &mut Self, buffer: u32, count: u32) {
        self.highlight_counts_mut().insert(buffer, count);
    }

    fn set_last_seen_msg(self: &mut Self, buffer: u32, msgid: u32) {
        self.last_seen_msg_mut().insert(buffer, msgid);
    }

    fn set_marker_line(self: &mut Self, buffer: u32, msgid: u32) {
        self.marker_line_mut().insert(buffer, msgid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::signalproxy::translation::Network;
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
            BufferSyncer::from_network(&mut get_network()),
            get_runtime()
        )
    }
}
