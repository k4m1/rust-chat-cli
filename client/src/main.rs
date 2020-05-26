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

    //just go read the server code 
    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match slient.read_exact(&mut buff) {
            Ok(_) => {
                let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                println!("msg recv {:?}", msg);
            },
            // ifyou woulf get a wouldblock error sever the connection with the server and throw hands
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                println!("Honetly if you get this one I dont know what happened")
                break;
            }
        }
            // if the msg is recv succ then clone it into bytes 
            // then put it in a buffer
            //thenm resize the buffer
            //then write all the buffers into the client
            //if that fails print that it failed
        math rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("Writing buffer to client failed");
                println!("message sent {:?"}, msg);
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvErrror::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    // ask for input loop for input print or get out
    println!("Say somthing:");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("failed to read stdin");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {break} 
    }
    println!("SMELL YA L8TR")
}
