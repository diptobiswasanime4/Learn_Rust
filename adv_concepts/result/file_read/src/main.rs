use std::{
    fs::File,
    io::{Error,Read}
};

fn read_file_content(file_path: &str) -> Result<String, Error> {
    let file_result = File::open(file_path);

    match file_result {
        Ok(mut file) => {
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(_) => Ok(content),
                Err(err) => Err(err),
            }
        }
        Err(err) => {
            Err(err)
        }
    }
}

fn main() {
    match read_file_content("file.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}
