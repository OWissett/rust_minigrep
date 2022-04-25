use std::env;
use std::process;

use minigrep::Config;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("\x1b[93mUnable to parse arguments: {}\x1b[0m", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("\x1b[93mApplication error: {}\x1b[0m", e);
        process::exit(1);
    }
}  

