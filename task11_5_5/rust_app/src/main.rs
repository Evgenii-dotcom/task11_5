use std::io::Write;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000")
        .expect("Failed to bind address");

    println!("Server listening on port 9000");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let message = "Hello from Rust";
                let response = format!(
                    "HTTP/1.1 200 OK\r\n\
                     Content-Length: {}\r\n\
                     Connection: close\r\n\
                     \r\n\
                     {}",
                    message.len(),
                    message
                );
                
                let _ = stream.write(response.as_bytes());
                let _ = stream.flush(); 
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}