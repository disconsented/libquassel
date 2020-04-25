use failure::Error;

extern crate libquassel;
use libquassel::client;

extern crate tokio;
extern crate pretty_env_logger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    pretty_env_logger::init();

//    let mut client = client::Client::<tokio::net::TcpStream>::connect(
//        "cocaine.farm",
//        4242,
//        true,
//    ).await.unwrap();

    let mut client = client::Client::<tokio_tls::TlsStream<tokio::net::TcpStream>>::connect_tls(
        "cocaine.farm",
        4242,
        true,
    ).await.unwrap();

    client.run().await;
//    client.login("audron", "audron", client_init);

    Ok(())
} // the stream is closed here
