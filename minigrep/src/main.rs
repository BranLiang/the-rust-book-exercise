use std::{env, process};
use minigrep::run;
use minigrep::config;

fn main() {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("An application error: {}", e);
        process::exit(1);
    }
}


