use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect(); 
    // let query = &args[1];
    // let filename = &args[2];
    // println!("{} {}", query, filename);

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing the arguments {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
        
    }
}

