use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::collections::HashMap;

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

fn parse_request(mut stream: &TcpStream) -> (String, String, HashMap<String, String>, String) {
	let mut request_line = String::new();
	let mut headers = HashMap::new();
	let mut headers_complete = false;
	let mut body_lines = Vec::new();
	'tcp_loop: loop {
		let mut buffer = [0; 1024];
		let n = stream.read(&mut buffer).expect("Can't read HTTP Request");
		let raw_request = String::from_utf8_lossy(&buffer[..n]);
		let mut request = raw_request.lines();

		// First line of a HTTP request is special.  Store it here and parse at the end
		if request_line.is_empty() {
			request_line = request.next().expect("HTTP Request needs a first line").to_string();
		}

		if !headers_complete {
			// Following lines are headers key/value pairs
			'header_loop: while let Some(header_line) = request.next() {
				if header_line.is_empty() {
					headers_complete = true;
					break 'header_loop;
				}
				let kv: Vec<&str> = header_line.trim().splitn(2, ":").map(|val| val.trim()).collect();
				match kv.len() {

					// Only a key and no value seems like a problem with the HTTP Client
					1 => println!("Ignoring invalid header \"{}\" (Length: {}).", header_line.to_string(), header_line.len()),

					// A key plus value is what we expect, save to header hashmap
					2 => drop(headers.insert(kv[0].to_string(), kv[1].to_string())),

					// Any other number means we've messed up somehow
					_ => panic!("splitn returned too many values"),
				}
			}
			if !headers_complete {
				continue 'tcp_loop;
			}
		}

		/* Below is a fairly ropey implementaion of reading the body from a request
		  Needs more testing, particularly of larger requests and range of network speeds
		  */
		if !headers.contains_key("Content-Length") {
			break 'tcp_loop;
		}
		let content_length = headers["Content-Length"].parse::<usize>().expect("Content-Length header should be an integer");

		while let Some(body_line) = request.next() {
			if body_line.is_empty() {
				break 'tcp_loop;
			}
			body_lines.push(body_line.to_string());
			if body_lines.join("\n").len() >= content_length {
				break 'tcp_loop;
			}
		}
	}

	let requestparts: Vec<&str> = request_line.splitn(3, " ").collect();
	let (method, path, _protocol) = (requestparts[0].to_string(), requestparts[1].to_string(), requestparts[2].to_string());
	if !headers.contains_key("User-Agent") {
		headers.insert("User-Agent".to_string(), "-".to_string());
	}
	let body = body_lines.join("\n");
	return (method, path, headers, body);
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
	let (method, path, headers, _) = parse_request(&stream);
	println!("Request: {}, {}, {}", method, path, headers["User-Agent"]);
	send_response(stream);
}