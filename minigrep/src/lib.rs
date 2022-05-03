pub mod config;

use std::{error::Error, fs};

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(config.query.as_str(), contents.as_str())
    } else {
        search_case_insensitive(config.query.as_str(), contents.as_str())
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| {
        line.contains(query)
    }).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| {
        line.to_lowercase().contains(&query.to_lowercase())
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust
safe, fast, productive.
Pick three
Ductap.";

        assert_eq!(search(query, contents), vec!["safe, fast, productive."]);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust
safe, trust, productive.
Pick three.";

        assert_eq!(search_case_insensitive(query, contents), vec!["Rust", "safe, trust, productive."]);
    }
}