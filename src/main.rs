mod consts;
mod net;

mod protocol;

#[macro_use]
mod util;

#[cfg(test)]
mod tests;

//use util::Hex;
use protocol::primitive::{String, StringList};
use protocol::message::{ClientInit};

fn main() -> std::io::Result<()> {
    let mut server = net::connect(
        "localhost",
        4242,
        false,
        false,
    )?;

    let mut features = StringList::new();
    features.push("SynchronizedMarkerLine".to_string());
    features.push("Authenticators".to_string());
    features.push("ExtendedFeatures".to_string());
    let client = ClientInit {
        client_version:String::from("Rust 0.0.0"),
        client_date: String::from("1579009211"),
        feature_list: features,
        client_features: 0x00008000,
    };
    server.login("audron", "audron", client);

    Ok(())
} // the stream is closed here
