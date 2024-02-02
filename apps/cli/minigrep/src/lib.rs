use std::fs;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.")
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) {
    let file_content = fs::read_to_string(config.file_path.clone())
    .expect("Unable to read from file.");

    println!("{}", file_content);

    println!("Searching for {} in file path {}", config.query, config.file_path);
}