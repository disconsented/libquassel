use crate::message::signalproxy::MessageType;
use std::collections::HashMap;

// use default_macro::default;
// #[default(crate::message::signalproxy::objects::BufferSyncerClient)]
pub struct BufferSyncer {
    pub activities: HashMap<u32, MessageType>,
    pub highlight_counts: HashMap<u32, u32>,
    pub last_seen_msg: HashMap<u32, u32>,
    pub marker_line: HashMap<u32, u32>,
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
