use failure::Error;

extern crate libquassel;

extern crate tokio;
extern crate pretty_env_logger;

use libquassel::primitive::*;
use libquassel::message::*;
use libquassel::client::*;

use tokio::io::{AsyncRead, AsyncWrite};
use core::marker::Unpin;
use futures::SinkExt;
use std::future::Future;

use log::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();

//    let mut client = client::Client::<tokio::net::TcpStream>::connect(
//        "cocaine.farm",
//        4242,
//        true,
//    ).await.unwrap();

    let username = std::env::args().nth(1).expect("no username given");
    let password = std::env::args().nth(2).expect("no password given");

   let funcs = Funcs {
       init: InitFuncs {
           client_init_ack,
           client_init_reject,
           client_login_ack,
           client_login_reject,
           session_init
       },
       message: MessageFuncs {
           sync_message,
           rpc_call,
           init_request,
           init_data,
           heart_beat,
           heart_beat_reply
       }
   };

    let mut client = Client::<tokio_tls::TlsStream<tokio::net::TcpStream>>::connect_tls(
        "cocaine.farm",
        4242,
        true,
        User {
            name: username,
            password,
        },
        //funcs,
    ).await.unwrap();

    client.run().await;

    Ok(())
}

async fn client_init_ack<T: AsyncRead + AsyncWrite + Unpin, F: Future>(client: &mut Client<T, F>, item: VariantMap) {
    use libquassel::{HandshakeSerialize, HandshakeDeserialize};

    info!(target: "init", "Initialization successfull");
    info!(target: "login", "Starting Login");
    let login = ClientLogin {user: client.user.name.clone(), password: client.user.password.clone()};
    client.stream.send(login.serialize().unwrap()).await.unwrap();
}

async fn client_init_reject<T: AsyncRead + AsyncWrite + Unpin, F: Future>(client: Client<T, F>, item: VariantMap) {
    error!(target: "init", "Initialization failed: {:?}", item);
}

async fn client_login_ack<T: AsyncRead + AsyncWrite + Unpin, F: Future>(client: Client<T, F>, item: VariantMap) {
    info!(target: "login", "Login successfull");
}

async fn client_login_reject<T: AsyncRead + AsyncWrite + Unpin, F: Future>(client: Client<T, F>, item: VariantMap) {
    error!(target: "login", "Login failed: {:?}", item);
}

async fn session_init<T: AsyncRead + AsyncWrite + Unpin, F: Future>(client: &mut Client<T, F>, item: VariantMap) {
    info!(target: "login", "Session Initialization finished. Switching to Connected state");
    client.state = ClientState::Connected;
}

async fn sync_message<T: AsyncRead + AsyncWrite + Unpin, F: Future>(client: Client<T, F>, item: SyncMessage) { unimplemented!() }
async fn rpc_call<T: AsyncRead + AsyncWrite + Unpin, F: Future>(client: Client<T, F>, item: RpcCall) { unimplemented!() }
async fn init_request<T: AsyncRead + AsyncWrite + Unpin, F: Future>(client: Client<T, F>, item: InitRequest) { unimplemented!() }
async fn init_data<T: AsyncRead + AsyncWrite + Unpin, F: Future>(client: Client<T, F>, item: InitData) { unimplemented!() }
async fn heart_beat<T: AsyncRead + AsyncWrite + Unpin, F: Future>(client: Client<T, F>, item: HeartBeat) { unimplemented!() }
async fn heart_beat_reply<T: AsyncRead + AsyncWrite + Unpin, F: Future>(client: Client<T, F>, item: HeartBeatReply) { unimplemented!() }
