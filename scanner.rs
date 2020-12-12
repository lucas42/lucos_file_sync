use std::fs;
use std::path::Path;

pub fn scan() {
	get_files("/mnt/documents")
}
fn get_files<P>(dir_path: P) 
where
    P: AsRef<Path> {

    let files = fs::read_dir(dir_path).unwrap().filter_map(Result::ok);

    for file in files {
    	if file.file_name().into_string().unwrap().starts_with(".") {
    		continue;
    	}
    	let file_metadata = file.metadata().unwrap();
    	if file_metadata.is_dir() {
    		get_files(file.path());
    	}
    	if file_metadata.is_file() {
    		println!("File: {}", file.path().display());
    	}
        
    }
}