use std::fs;
use std::time::UNIX_EPOCH;

pub fn scan() {
	get_files("documents", "");
}
fn get_files(namespace: &str, rel_dir: &str) {
	let full_path = format!("/mnt/{}/{}", namespace, rel_dir);
	let files = fs::read_dir(&full_path).unwrap().filter_map(Result::ok);

	for file in files {
		let file_name = file.file_name().into_string().unwrap();
		if file_name.starts_with(".") {
			continue;
		}
		let rel_path = match rel_dir {
			"" => file_name,
			_ => format!("{}/{}", rel_dir, file_name)
		};
		let file_metadata = file.metadata().unwrap();
		if file_metadata.is_dir() {
			get_files(namespace, &rel_path);
		}
		let modified = file_metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
		if file_metadata.is_file() {
			println!("{}: {}, {}", namespace, rel_path, modified);
		}
	}
}