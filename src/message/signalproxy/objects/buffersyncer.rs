use std::{collections::HashMap, convert::TryInto};

use num_traits::{FromPrimitive, ToPrimitive};

use itertools::Itertools;

use crate::{
    message::signalproxy::Network,
    primitive::{MessageType, Variant, VariantList},
};

#[derive(Debug, Clone, PartialEq)]
pub struct BufferSyncer {
    pub activities: HashMap<i32, MessageType>,
    pub highlight_counts: HashMap<i32, i32>,
    pub last_seen_msg: HashMap<i32, i64>,
    pub marker_line: HashMap<i32, i64>,
}

impl Network for BufferSyncer {
    type Item = VariantList;

    fn to_network(&self) -> Self::Item {
        let mut res = Self::Item::new();

        res.push(Variant::ByteArray(s!("Activities")));
        res.push(Variant::VariantList({
            let mut res = VariantList::new();

            self.activities.iter().for_each(|(k, v)| {
                res.push(Variant::i32(*k));
                res.push(Variant::i32(v.to_i32().unwrap()));
            });

            res
        }));

        res.push(Variant::ByteArray(s!("HighlightCounts")));
        res.push(Variant::VariantList({
            let mut res = VariantList::new();

            self.highlight_counts.iter().for_each(|(k, v)| {
                res.push(Variant::i32(*k));
                res.push(Variant::i32(*v));
            });

            res
        }));

        res.push(Variant::ByteArray(s!("LastSeenMsg")));
        res.push(Variant::VariantList({
            let mut res = VariantList::new();

            self.last_seen_msg.iter().for_each(|(k, v)| {
                res.push(Variant::i32(*k));
                res.push(Variant::i64(*v));
            });

            res
        }));

        res.push(Variant::ByteArray(s!("MarkerLines")));
        res.push(Variant::VariantList({
            let mut res = VariantList::new();

            self.marker_line.iter().for_each(|(k, v)| {
                res.push(Variant::i32(*k));
                res.push(Variant::i64(*v));
            });

            res
        }));

        res
    }

    fn from_network(input: &mut Self::Item) -> Self {
        let mut i = input.iter().cycle();

        i.position(|x| *x == crate::primitive::Variant::ByteArray(s!("Activities")))
            .unwrap();
        let activities: VariantList = i.next().unwrap().try_into().unwrap();
        let activities = activities
            .iter()
            .batching(|it| match it.next() {
                None => None,
                Some(x) => match it.next() {
                    None => None,
                    Some(y) => Some((
                        x.try_into().unwrap(),
                        MessageType::from_i32(y.try_into().unwrap()).unwrap(),
                    )),
                },
            })
            .collect();

        i.position(|x| *x == crate::primitive::Variant::ByteArray(s!("HighlightCounts")))
            .unwrap();
        let highlight_counts: VariantList = i.next().unwrap().try_into().unwrap();
        let highlight_counts = highlight_counts
            .iter()
            .batching(|it| match it.next() {
                None => None,
                Some(x) => match it.next() {
                    None => None,
                    Some(y) => Some((x.try_into().unwrap(), y.try_into().unwrap())),
                },
            })
            .collect();

        i.position(|x| *x == crate::primitive::Variant::ByteArray(s!("LastSeenMsg")))
            .unwrap();
        let last_seen_msg: VariantList = i.next().unwrap().try_into().unwrap();
        let last_seen_msg = last_seen_msg
            .iter()
            .batching(|it| match it.next() {
                None => None,
                Some(x) => match it.next() {
                    None => None,
                    Some(y) => Some((x.try_into().unwrap(), y.try_into().unwrap())),
                },
            })
            .collect();

        i.position(|x| *x == crate::primitive::Variant::ByteArray(s!("MarkerLines")))
            .unwrap();
        let marker_line: VariantList = i.next().unwrap().try_into().unwrap();
        let marker_line = marker_line
            .iter()
            .batching(|it| match it.next() {
                None => None,
                Some(x) => match it.next() {
                    None => None,
                    Some(y) => Some((x.try_into().unwrap(), y.try_into().unwrap())),
                },
            })
            .collect();

        Self {
            activities,
            highlight_counts,
            last_seen_msg,
            marker_line,
        }
    }
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
                1 => MessageType::None,
                2 => MessageType::None,
                3 => MessageType::None,
                4 => MessageType::None,
                5 => MessageType::None,
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
