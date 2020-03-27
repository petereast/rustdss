mod connection;
mod constants;
mod request;

fn main() -> Result<(), std::io::Error> {
    let core = rustdss_core::Core::start();
    connection::Connection::start(core.get_sender())?;

    Ok(())
}
