use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config { query, file_path, ignore_case })
    }
}

#[allow(dead_code)]
pub fn start() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let file_contents = fs::read_to_string(config.file_path)
        .unwrap_or_else(|err| {
            eprintln!("Couldn't read the file: {err}");
            process::exit(1);
        });

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &file_contents)
    } else {
        search(&config.query, &file_contents)
    };

    for line in result {
        println!("{line}");
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|x| x.contains(query)).collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    contents.lines().filter(|x| x.to_lowercase().contains(&query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "DucT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search_case_insensitive(query, contents));
    }
}