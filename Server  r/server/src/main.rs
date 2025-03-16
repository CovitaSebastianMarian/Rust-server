use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;




fn command_msg(input: &str,mut stream: &TcpStream) {
    stream.write_all(format!("MSG{}", input).as_bytes()).unwrap();
}
fn command_raw(mut stream: &TcpStream) -> String {
    stream.write_all("RAW".as_bytes()).unwrap();
    let mut buffer = [0; 512];
    let len = stream.read(&mut buffer).unwrap();
    let ret = String::from_utf8_lossy(&buffer[..len]);
    ret.as_ref().to_string()
}
fn command_end(mut stream: &TcpStream) {
    stream.write_all("END".as_bytes()).unwrap();
}

fn execute_command(msg: &str, stream: TcpStream) {
    let command = msg.split_whitespace().collect::<Vec<&str>>().join(" ");
    let command = command.as_str();
    match command {
        "sebux da-mi acces" => {
            command_msg("Enter password:", &stream);
            let password = command_raw(&stream);
            println!("{}", password);
            if password.trim() == "sebux" {
                command_msg("Acces permis!", &stream);
                command_msg(r#"Salutare ma bucur ca ai intrat aici!"#, &stream);
            }
            else  {
                command_msg("Acces denied!", &stream);
            }
            command_end(&stream);
        }
        _ => {
            println!("Command not found!");
            command_msg("Command not found!", &stream);
            command_end(&stream);
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(size) => {
                let message = String::from_utf8_lossy(&buffer[..size]);
                println!("Received: {}", message);
                execute_command(message.clone().as_ref(), stream.try_clone().unwrap());
            }
            Err(e) => {
                eprintln!("{}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port 8080");

    println!("Server running on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
