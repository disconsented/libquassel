use failure::Error;

extern crate libquassel;
use libquassel::client;

#[macro_use]
extern crate tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let mut client = client::Client::connect(
        "localhost",
        4242,
        false,
        true,
    ).await.unwrap();

    client.run().await;
//    client.login("audron", "audron", client_init);

    Ok(())
} // the stream is closed here
