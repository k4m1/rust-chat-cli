use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::syn::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    // start the server and non blocking, throw errs if failure
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("Failed to intitalize non-blocking");


    // vector to store all of our clients
    let mut clients = vec![];
    // create a channel and tell him 'hey bud im going to be sending some strings here that cool?'
    let (tx, rx) = mspec ::channel::<String>();
    // if let to desconstruct the result from server.accept (what allows us to accept connections)
    loop {
        if let ok((mut socket, addr)) = server.accept() {
            println!("client {} connected", addr);

            // clone the transmitter
            let tx = tx.clone();
            // copy client to be pushed into thread
            clients.push(socket.try_clone().expect("failed to clone client"));
        }
    }
}
