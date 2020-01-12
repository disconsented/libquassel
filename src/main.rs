mod consts;
mod net;
mod types;

#[macro_use]
mod util;

//use util::Hex;

fn main() -> std::io::Result<()> {
    let mut server = net::connect(
        "localhost",
        4242,
        false,
        false,
    )?;

    let client = types::handshake::ClientInit {
        client_version: String::from("Rust 0.0.0"),
        build_date: String::from("today"),
        client_features: 0x00000000,
        feature_list: types::StringList::new()
    };
    server.login("audron", "audron", client);

    Ok(())
} // the stream is closed here
