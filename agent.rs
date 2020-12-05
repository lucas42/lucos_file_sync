use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn main() {
	let port = env::var("PORT").expect("Environment Varible PORT must be set")
		.parse::<u16>().expect("Environment Varible PORT must be an integer");

	let listener = TcpListener::bind(("0.0.0.0", port)).expect("Server failed");
	println!("Running server; listening on {}", listener.local_addr().unwrap());

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {

				// Spawn each connection in a separate thread
				thread::spawn(|| {
					handle_connection(stream);
				});
			}
			Err(error) => {
				println!("Connection failure: {}", error)
			}
		}
	}
}

fn parse_request(mut stream: &TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Can't read HTTP Request");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);
}

fn send_response(mut stream: TcpStream) {
	let response = "HTTP/1.1 200 OK
Content-Type: text/html; charset=UTF-8

<html><body>Hello world</body></html>
";
	stream.write(response.as_bytes()).expect("Failed sending response");
	println!("Response sent");
}


fn handle_connection(stream: TcpStream) {
	parse_request(&stream);
	send_response(stream);
}