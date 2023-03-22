use std::fs;
use std::error::Error;
use std::env;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(v) => v,
            None => return Err("did not recieved query")
        };
        let filename = match args.next() {
            Some(v) => v,
            None => return Err("did not recieved filename")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!("{}",config.case_sensitive);
    let result = if config.case_sensitive {
        search_case_senistive(&config.query, &contents)
    } else {
        search_case_insenistive(&config.query, &contents)
    };

    for line in result{
        println!("{}", line);
    }
    Ok(())
}


pub fn search_case_senistive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = vec![];
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }
    // result
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insenistive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast and productive
yoyoy";

        assert_eq!(vec!["safe, fast and productive"], search_case_senistive(query, contents));
    }

    #[test]
    fn test_case_insensitive() {
        let query = "RUst";
        let contents = "\
Rust:
safe, fast and productive
yoyoy";

        assert_eq!(vec!["Rust:"], search_case_insenistive(query, contents));
        
    }
}