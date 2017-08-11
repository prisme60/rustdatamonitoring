use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;
use std::thread;
use std::thread::sleep;
use std::thread::JoinHandle;
use std::fs::remove_file;
use std::os::unix::net::UnixListener;

pub struct Server {
    listener : UnixListener,
}

impl Server {
    pub fn create_server_thread(socket_path : &'static str) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut serv = Server::new(socket_path);
            serv.receive_or_wait();
        })
    }
    
    pub fn new(socket_path : &str) -> Server {
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
    
    pub fn receive_or_wait(& mut self) {
        println!("Server started, waiting for clients");
       
        // accept connections and process them
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    /* connection succeeded */
                    println!("Connection succeeded {:?}", stream);
                    stream.write_all(b"hello world\n").unwrap();
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
