mod consts;
mod client;
mod protocol;

#[macro_use]
mod util;

#[macro_use]
extern crate failure;

#[cfg(test)]
mod tests;

//use util::Hex;
use protocol::primitive::{String, StringList};
use protocol::message::{ClientInit};

use failure::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let mut client = client::Client::connect(
        "localhost",
        4242,
        false,
        true,
    ).await.unwrap();

    let mut features = StringList::new();
    features.push("SynchronizedMarkerLine".to_string());
    features.push("Authenticators".to_string());
    features.push("ExtendedFeatures".to_string());
    let client_init = ClientInit {
        client_version:String::from("Rust 0.0.0"),
        client_date: String::from("1579009211"),
        feature_list: features,
        client_features: 0x00008000,
    };

    client.handler().await?;
//    client.login("audron", "audron", client_init);

    Ok(())
} // the stream is closed here
