use std::thread;
mod server;
mod scanner;

fn main() {
	let _ = thread::Builder::new().name("file_sync_scanner".to_string()).spawn(|| {
		scanner::scan();
	});
    server::start();
}