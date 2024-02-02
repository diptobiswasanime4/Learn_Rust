use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else((|err| {
        println!("Unable to parse args. {err}");
        process::exit(1);
    }));

    minigrep::run(config);

    dbg!(args);
}