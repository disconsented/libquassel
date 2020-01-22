mod consts;

#[cfg(features = "client")]
mod client;

mod protocol;

#[macro_use]
mod util;

#[macro_use]
extern crate failure;

#[cfg(test)]
mod tests;

use failure::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let mut client = client::Client::connect(
        "localhost",
        4242,
        false,
        false,
    ).await.unwrap();

    client.run().await;
//    client.login("audron", "audron", client_init);

    Ok(())
} // the stream is closed here
