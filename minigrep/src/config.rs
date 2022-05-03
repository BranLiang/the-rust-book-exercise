use std::env::{self, Args};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        // First argument is the program name, ignore it here 
        args.next();

        let query = match args.next() {
            Some(value) => value,
            None => return Err("missing query")
        };

        let filename = match args.next() {
            Some(value) => value,
            None => return Err("missing filename")
        };

        Ok(Config {
            query,
            filename,
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}
