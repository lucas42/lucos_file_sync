use std::env;

fn main() {
	let port = env::var("PORT").expect("Environment Varible PORT must be set")
		.parse::<u16>().expect("Environment Varible PORT must be an integer");

	println!("Running Agent {}", port);
}
