# libquassel
Native rust implementation of the Quassel protocol and library functions

## Features
| Feature              | Description                              |
|----------------------+------------------------------------------|
| client               | Enable client side trait implementations |
| server               | Enable server side trait implementations |
| framing              | Enable support for tokio's [codec::Framed](https://docs.rs/tokio-util/latest/tokio_util/codec/struct.Framed.html) |
| all-quassel-features | enable all protocol features             |
| long-message-id      | Serialize message IDs as i64             |
| long-time            | Serialize Message Time as i64            |
| rich-messages        | add avatar url and real name to messages |
| sender-prefixes      | Show prefixes for senders in backlog     |
| authenticators       | Support for exchangeable auth backends   |
