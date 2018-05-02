use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead, BufWriter, Write};

fn handle(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    loop {
        let mut message = String::new();
        reader.read_line(&mut message).expect("Failed to read");
        match &*message.trim_right() {
            "quit" => {
                break;
            }
            _ => {
                writer.write(message.as_bytes()).unwrap();
                let _ = writer.flush();
            }
        }
    }
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client!");
                handle(stream);
            }
            Err(_e) => { /* connection failed */ }
        }
    }
}