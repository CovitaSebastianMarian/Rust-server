use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;


struct Client {
    stream: TcpStream,
    id: usize,
}
impl Client {
    fn new(stream: TcpStream, id: usize) -> Client {
        Client { stream, id }
    }
    fn run(&mut self, sender: Sender<String>) {
        let mut buffer = [0; 512];

        loop {
            match self.stream.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    let msg = format!("Client {}: {}", self.id, String::from_utf8_lossy(&buffer[..n]));
                    sender.send(msg).unwrap();
                }
                _ => {
                    println!("Client {} s-a deconectat.", self.id);
                    break;
                }
            }
        }
    }
}

struct Server {
    listener: TcpListener,
    clients: Arc<Mutex<Vec<TcpStream>>>,
}
impl Server {
    fn new(addr: &str) -> Server {
        Server {
            listener: TcpListener::bind(addr).expect("Eroare la pornirea serverului!"),
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }
    fn run(&self) {
        let clients_clone = Arc::clone(&self.clients);
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        thread::spawn(move || {
            for msg in rx.iter() {
                println!("[{}]", msg.trim());
                let clients_guard = clients_clone.lock().unwrap();
                for mut client in clients_guard.iter() {
                    let _ = client.write_all(msg.trim().as_bytes());
                }
            }
        });

        let mut client_id = 0;

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let stream_clone = stream.try_clone().expect("Eroare la clonare stream!");
                    self.clients.lock().unwrap().push(stream_clone);
                    let sender_clone = tx.clone();

                    thread::spawn(move || {
                        let mut client = Client::new(stream, client_id);
                        client.run(sender_clone);
                    });
                    client_id += 1;
                }
                Err(e) => {
                    println!("[EROARE]: {}", e);
                }
            }
        }



        thread::spawn(move || {
            let mut input = String::new();

            while io::stdin().read_line(&mut input).is_ok() {
                
            }


        });



    }
}

fn main() {
    let server = Server::new("127.0.0.1:8000");
    server.run();
}
