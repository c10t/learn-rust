use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // &'static str is the type of string literals: “The Static Lifetime” section in Chapter 10
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // panic!("not enough arguments");
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// Box<dyn Error> means the function will return a type that implements the Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}