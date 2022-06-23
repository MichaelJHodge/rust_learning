//Minigrep Rust Project//

//This function from the std lib returns an iterator of the CLI args
//that were given to minigrep. we use two :: because the function is
//nested in more than one module.
use std::env;
use std::process;

//This brings the Config type from the library crate into the binary crate's scope.

use minigrep::Config;

fn main() {
    //This uses collect to turn the iterator into a vector containing
    //all the values produced by the iterator. We specify that we
    //want to create a collection specifically of strings since there are many
    //types of collections we could create.

    //Updated to handle the Result type
    let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //We use if let to check whther run returns an Err value or not and exit if
    //we do.
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
