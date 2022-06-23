use std::error::Error;

//Allows us to handle files.
use std::fs;

//Brings the env module into scope which allows us to use functions
//pertaining to environment variables.
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

//We tell Rust that the data returned by the search function will live as long as
//the data passed into the search function in the contents argument. This is important!
//The data referenced by a slice needs to be valid for the reference to be valid; if the
//compiler assumes we’re making string slices of query rather than contents, it will do
//its safety checking incorrectly.

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //this code uses the filter adaptor to keep
    //only the lines that line.contains(query) returns true for. We then
    //collect the matching lines into another vector with collect.
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

//Basically the same as the above function, but instead we lowercase the query and
//each line so whatever the case of the input args, they will be the same case when we check
//if the lines contain the query.

//When we pass query as an argument to the contains method now, we need to add an
//ampersand because the signature of contains is defined to take a string slice.

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

//This means that the trait object Box<dyn Error> will return a type that
//implements the Error trait, but we do not need to specify what type of error
//the turn value will be.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

//Borrows these values (args) from main. In this case we clone the values
//and make our own copies, which is more expensive from a performance standpoint
//but it also means we don't need to keep track of reference lifetimes and in this case
//it makes sense to sacrifice for some simplicity.

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        if args.len() > 3 {
            return Err("Too many arguments");
        }

        //Keeps track of the new env variable we want. The is_err method checks whether or not
        //it's an error and therefore unset - meaning we should do a case-sensitive search. If it is
        //set to anything, it returns false and the progrma performs a case-insensitive search as usual.
        //we dont need the value of our env variable, we just need to see if it's set or unset.

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
