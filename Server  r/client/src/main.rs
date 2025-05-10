use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::thread; 

struct Client {
    stream: TcpStream,
}
impl Client {
    fn new(addr: &str) -> Client {
        let stream = TcpStream::connect(addr).expect("Eroare la conectarea la server!");
        Client { stream }
    }
    fn listen(&mut self) {
        let mut stream_clone = self
            .stream
            .try_clone()
            .expect("Eroare la clonarea stream-ului!");

        thread::spawn(move || {
            let mut buffer = [0; 512];
            loop {
                match stream_clone.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        println!(">> {}", String::from_utf8_lossy(&buffer[..n]));
                    }
                    _ => {
                        println!("Serverul s-a închis!");
                        break;
                    }
                }
            }
        });
    }
    fn send(&mut self) {
        let mut input = String::new();
        while io::stdin().read_line(&mut input).is_ok() {
            self.stream
                .write_all(input.as_bytes())
                .expect("Eroare la trimiterea mesajului!");
            input.clear();
        }
    }
}

fn main() {
    let mut client = Client::new("127.0.0.1:8000");
    client.listen();
    client.send();
}
