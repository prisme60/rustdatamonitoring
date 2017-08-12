//use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;
use std::thread;
use std::thread::sleep;
use std::thread::JoinHandle;
use std::fs::remove_file;
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;


pub struct Server {
    listener : UnixListener,
}

impl Server {
    pub fn create_server_thread(socket_path : &str) -> (JoinHandle<()>, Receiver<UnixStream>) {
        // Because it is not a static str, we have to use Arc (A thread-safe reference-counting pointer)
        let socket_path_share = Arc::new(socket_path.to_string());
        let socket_path_copy = Arc::clone(&socket_path_share);
        let (tx, rx) = channel::<UnixStream>();
        (thread::spawn(move || {
            let mut serv = Server::new(socket_path_copy.as_str());
            serv.receive(tx);
        }), rx)
    }
    
    fn new(socket_path : &str) -> Server {
        let socket = Path::new(socket_path);
    
        // Delete old socket if necessary
        if socket.exists() {
            let _ = remove_file(socket);
        }
        
        // Bind to socket
        match UnixListener::bind(&socket) {
            Err(_) => panic!("failed to bind socket"),
            Ok(listener) => {
                //listener.set_nonblocking(true).expect("Couldn't set non blocking");
                Server {
                    listener
                }
            },
        }
    }
    
    fn receive(& mut self, tx_channel : Sender<UnixStream>) {
        println!("Server started, waiting for clients");
       
        // accept connections and process them
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    /* connection succeeded */
                    println!("Connection succeeded {:?}", stream);
                    tx_channel.send(stream).unwrap();
                    //stream.write_all(b"hello world\n").unwrap();
                }
                Err(err) => {
                    /* connection failed */
                    println!("Connection failed : {}", err);
                    sleep(Duration::from_secs(5));
                }
            }
        }
    }
}
