mod connection;
mod constants;
mod core;
mod request;
mod transport;

fn main() -> Result<(), std::io::Error> {
    let core = core::Core::start();
    connection::Connection::start(core.get_sender())?;

    Ok(())
}
