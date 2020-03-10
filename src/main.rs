mod connection;
mod core;
mod request;
mod transport;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");

    let core = core::Core::start();
    connection::Connection::start(core.get_sender())?;

    Ok(())
}
