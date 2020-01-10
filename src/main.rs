mod consts;
mod net;
mod util;
mod types;

//use util::Hex;

fn main() -> std::io::Result<()> {
    let server = net::connect(
        "localhost",
        4242,
        false,
        false,
    )?;

    //server.login("audron", "audron");

    Ok(())
} // the stream is closed here
