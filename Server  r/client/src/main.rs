use std::io::{self, Read, Write};
use std::net::TcpStream;

fn execute_command(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 512];
        let len = stream.read(&mut buffer).unwrap();
        let message = String::from_utf8_lossy(&buffer[..len]);
        let message = message.as_ref();
        if message.starts_with("MSG") {
            print!("{}", &message[3..]);
            io::stdout().flush().unwrap();
            println!();
        } else if message.trim() == "END" {
            break;
        } else if message.trim() == "RAW" {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            stream.write_all(input.as_bytes()).unwrap();
        }
    }
}

fn main() {
    let mut client =
        TcpStream::connect("127.0.0.1:8080").expect("Eroare la conectarea cu serverul!");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "" {
            client.write_all(input.trim().as_bytes()).unwrap();
            execute_command(client.try_clone().unwrap());
        }
    }
}
