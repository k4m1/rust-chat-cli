use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::syn::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("Failed to intitalize non-blocking")
}
