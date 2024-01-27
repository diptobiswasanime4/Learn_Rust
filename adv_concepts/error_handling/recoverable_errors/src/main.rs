use std::fs::File;

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let file_found_result = File::open("hello.txt");

    let file_found = match file_found_result {
        Ok(file) => println!("File found. File: {:?}", file),
        Err(error) => panic!("File not found! Error: {:?}", error),
    };
}