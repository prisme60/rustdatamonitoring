use std::{
    fs::remove_file,
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{sleep, spawn, JoinHandle},
    time::Duration,
};

pub struct Server {
    listener: UnixListener,
}

impl Server {
    pub fn create_server_thread(socket_path: &str) -> (JoinHandle<()>, Receiver<UnixStream>) {
        let socket_path_copy = socket_path.to_string();
        let (tx, rx) = channel::<UnixStream>();
        (
            spawn(move || {
                let mut serv = Server::new(socket_path_copy.as_str());
                serv.receive(tx);
            }),
            rx,
        )
    }

    fn new(socket_path: &str) -> Server {
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
                Server { listener }
            }
        }
    }

    fn receive(&mut self, tx_channel: Sender<UnixStream>) {
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
