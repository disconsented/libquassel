use tokio::io::{AsyncRead, AsyncWrite};
use tokio_test::task;
use tokio_test::{
    assert_err, assert_ok, assert_pending, assert_ready, assert_ready_err, assert_ready_ok,
};
use tokio_util::codec::*;

use bytes::{BufMut, Bytes, BytesMut};
use futures::{pin_mut, Sink, Stream};

use std::collections::VecDeque;
use std::io;
use std::pin::Pin;
use std::task::Poll::*;
use std::task::{Context, Poll};

use flate2::Compress;
use flate2::Compression;
use flate2::Decompress;
use flate2::FlushCompress;
use flate2::FlushDecompress;

use crate::protocol::frame::QuasselCodec;

macro_rules! mock {
    ($($x:expr,)*) => {{
        let mut v = VecDeque::new();
        v.extend(vec![$($x),*]);
        Mock { calls: v }
    }};
}

macro_rules! assert_next_eq {
    ($io:ident, $expect:expr) => {{
        task::spawn(()).enter(|cx, _| {
            let res = assert_ready!($io.as_mut().poll_next(cx));
            match res {
                Some(Ok(v)) => assert_eq!(v, $expect.as_ref()),
                Some(Err(e)) => panic!("error = {:?}", e),
                None => panic!("none"),
            }
        });
    }};
}

macro_rules! assert_next_pending {
    ($io:ident) => {{
        task::spawn(()).enter(|cx, _| match $io.as_mut().poll_next(cx) {
            Ready(Some(Ok(v))) => panic!("value = {:?}", v),
            Ready(Some(Err(e))) => panic!("error = {:?}", e),
            Ready(None) => panic!("done"),
            Pending => {}
        });
    }};
}

macro_rules! assert_next_err {
    ($io:ident) => {{
        task::spawn(()).enter(|cx, _| match $io.as_mut().poll_next(cx) {
            Ready(Some(Ok(v))) => panic!("value = {:?}", v),
            Ready(Some(Err(_))) => {}
            Ready(None) => panic!("done"),
            Pending => panic!("pending"),
        });
    }};
}

macro_rules! assert_done {
    ($io:ident) => {{
        task::spawn(()).enter(|cx, _| {
            let res = assert_ready!($io.as_mut().poll_next(cx));
            match res {
                Some(Ok(v)) => panic!("value = {:?}", v),
                Some(Err(e)) => panic!("error = {:?}", e),
                None => {}
            }
        });
    }};
}

// ======================
// =====    Test    =====
// ======================

#[test]
pub fn read_single_frame() {
    let io = FramedRead::new(
        mock! {
            data(b"\x00\x00\x00\x09abcdefghi"),
        },
        QuasselCodec::new(),
    );
    pin_mut!(io);

    assert_next_eq!(io, b"abcdefghi");
    assert_done!(io);
}

#[test]
pub fn read_multi_frame() {
    let mut d: Vec<u8> = vec![];
    d.extend_from_slice(b"\x00\x00\x00\x09abcdefghi");
    d.extend_from_slice(b"\x00\x00\x00\x03123");
    d.extend_from_slice(b"\x00\x00\x00\x0bhello world");

    let io = FramedRead::new(
        mock! {
            data(&d),
        },
        QuasselCodec::new(),
    );
    pin_mut!(io);

    assert_next_eq!(io, b"abcdefghi");
    assert_next_eq!(io, b"123");
    assert_next_eq!(io, b"hello world");
    assert_done!(io);
}

#[test]
pub fn read_single_frame_compressed() {
    let io = FramedRead::new(
        mock! {
            data(b"\x78\x9c\x63\x60\x60\xe0\x4c\x4c\x4a\x4e\x49\x4d\x4b\xcf\xc8\x04\x00\x11\xec\x03\x97"),
        },
        QuasselCodec::builder().compression(true).new_codec(),
    );
    pin_mut!(io);

    assert_next_eq!(io, b"abcdefghi");
    assert_done!(io);
}

// TODO shit doens't work for whatever reason
// #[test]
// pub fn read_multi_frame_compressed() {
//     let io = FramedRead::new(
//         mock! {
//             data(
//                 b"\x78\x9c\x63\x60\x60\xe0\x4c\x4c\x4a\x4e\x49\x4d\x4b\xcf\xc8\x04\x00\x11\xec\x03\x97\x78\x9c\x63\x60\x60\x60\x36\x34\x32\x06\x00\x01\x3d\x00\x9a\x78\x9c\x63\x60\x60\xe0\xce\x48\xcd\xc9\xc9\x57\x28\xcf\x2f\xca\x49\x01\x00\x1a\x93\x04\x68",
//             ),
//         },
//         QuasselCodec::builder().compression(true).new_codec(),
//     );
//     pin_mut!(io);
//
//     assert_next_eq!(io, b"abcdefghi");
//     assert_next_eq!(io, b"123");
//     assert_next_eq!(io, b"hello world");
//     assert_done!(io);
// }

// ======================
// ===== Test utils =====
// ======================

struct Mock {
    calls: VecDeque<Poll<io::Result<Op>>>,
}

enum Op {
    Data(Vec<u8>),
    Flush,
}

use self::Op::*;

impl AsyncRead for Mock {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        dst: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        match self.calls.pop_front() {
            Some(Ready(Ok(Op::Data(data)))) => {
                debug_assert!(dst.len() >= data.len());
                dst[..data.len()].copy_from_slice(&data[..]);
                Ready(Ok(data.len()))
            }
            Some(Ready(Ok(_))) => panic!(),
            Some(Ready(Err(e))) => Ready(Err(e)),
            Some(Pending) => Pending,
            None => Ready(Ok(0)),
        }
    }
}

impl AsyncWrite for Mock {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        src: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        match self.calls.pop_front() {
            Some(Ready(Ok(Op::Data(data)))) => {
                let len = data.len();
                assert!(src.len() >= len, "expect={:?}; actual={:?}", data, src);
                assert_eq!(&data[..], &src[..len]);
                Ready(Ok(len))
            }
            Some(Ready(Ok(_))) => panic!(),
            Some(Ready(Err(e))) => Ready(Err(e)),
            Some(Pending) => Pending,
            None => Ready(Ok(0)),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        match self.calls.pop_front() {
            Some(Ready(Ok(Op::Flush))) => Ready(Ok(())),
            Some(Ready(Ok(_))) => panic!(),
            Some(Ready(Err(e))) => Ready(Err(e)),
            Some(Pending) => Pending,
            None => Ready(Ok(())),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Ready(Ok(()))
    }
}

impl<'a> From<&'a [u8]> for Op {
    fn from(src: &'a [u8]) -> Op {
        Op::Data(src.into())
    }
}

impl From<Vec<u8>> for Op {
    fn from(src: Vec<u8>) -> Op {
        Op::Data(src)
    }
}

fn data(bytes: &[u8]) -> Poll<io::Result<Op>> {
    Ready(Ok(bytes.into()))
}

fn flush() -> Poll<io::Result<Op>> {
    Ready(Ok(Flush))
}
