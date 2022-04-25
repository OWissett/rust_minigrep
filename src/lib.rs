use std::error::Error;
use std::fs;
use std::env;

/// Structs
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        
        args.next(); // skip the first arg (calling alias)

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing file name"),
        };
        
        let case_insensitive;

        match args.next() {
            Some(arg) => {
                if arg == "-i" {
                    case_insensitive = true;
                } else {
                    return Err("Unknown argument given!");
                }
            },
            None => { case_insensitive = false }
        }

        let case_insensitive = 
            !(env::var("CASE_INSENSITIVE").is_err()) || case_insensitive;
    
        Ok(Config {query, filename, case_insensitive})
    }
}

/// Runs the minigrep application
/// 
/// # Example
/// 
/// ```
/// use minigrep::{run, Config};
/// 
/// let conf = Config{query: "foo".to_string(), 
///                   filename: "foo bar".to_string(),
///                   case_insensitive: false};
/// run(conf); // prints "foo" 
/// 
/// let conf = Config{query: "Foo".to_string(), 
///                   filename: "foo bar".to_string(),
///                   case_insensitive: false};
/// run(conf); // prints nothing, as Foo and foo are not the same
/// 
/// let conf = Config{query: "Foo".to_string(), 
///                   filename: "foo bar".to_string(),
///                   case_insensitive: true};
/// 
/// run(conf); // print "foo" since case is ignored
/// 
/// ```
/// 
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(()) // we all good...
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

/// Unit tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive,
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive,"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive,
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }


}