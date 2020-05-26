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

            //create the thread and a mutable buff inside it and inside the buffer start a loop
            // the buffer contains a vec with 0's and the $MSG_SIZE
            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];
                // thus wull read our msg into the buffer
                //convert the boi into an ittr
                //take all non white space charcters and convert them into a real string
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("Invalid utf-8 msg");
                        // yo who sent that if if it failed let me know
                        println!( "{}: {:?}", addr, msg);
                        tx.send(msg).expect("failed to send msg to rx");
                    },
                    Err(ref err) if er.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println("closing connection with: {}", addr);
                        break;
                    }
                }
            });
        }
    }
}
