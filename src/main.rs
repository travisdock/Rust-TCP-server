use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 256;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() {
    println!("Starting server...");
    match TcpListener::bind(LOCAL) {
        Ok(server) => {
            server.set_nonblocking(true).expect("failed to initialize non-blocking");
            println!("Ready to accept connections at: {}", server.local_addr().unwrap());

            let mut clients = vec![];
            let (tx, rx) = mpsc::channel::<Vec<u8>>();
            loop {
                if let Ok((mut socket, addr)) = server.accept() {
                    println!("Client {} connected", addr);

                    let tx = tx.clone();
                    clients.push(socket.try_clone().expect("failed to clone client"));

                    thread::spawn(move || loop {
                        let mut buff = vec![0; MSG_SIZE];

                        match socket.read_exact(&mut buff) {
                            Ok(_) => {
                                // println!("{}: {:?}", addr, buff);
                                tx.send(buff).expect("failed to send msg to rx");
                            },
                            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                            Err(_) => {
                                println!("closing connection with: {}", addr);
                                break;
                            }
                        }

                        sleep();
                    });
                }

                if let Ok(msg) = rx.try_recv() {
                    clients = clients.into_iter().filter_map(|mut client| {
                        let mut buff = msg.clone();
                        buff.resize(MSG_SIZE, 0);

                        client.write_all(&buff).map(|_| client).ok()
                    }).collect::<Vec<_>>();
                }

                sleep();
            }
        },
        Err(e) => {
          println!("Could not start server because of error: \"{}\"", e)
        }
    }
}
