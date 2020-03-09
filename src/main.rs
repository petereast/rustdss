mod connection;
mod core;
mod request;
mod transport;

fn main() {
    println!("Hello, world!");

    core::Core::start();
    connection::Connection::start();
}
