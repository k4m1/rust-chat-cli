use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mspc::{Self, TryRecvError};
use std::thread;
use std::time::Duration;

const LOCAL: &str = "127.0.0.1:6000"
const MSG_SIZE: usize = 32; 

fn main() {
    //create mutable Tcpstream called client and if we fail to connect throw an err
    // do the same for non blocking
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to initate non-blocking");

    // like in server we make a channel for strings
    let (tx, rx) = mspc::channel::<String>();




}
