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
    

}
