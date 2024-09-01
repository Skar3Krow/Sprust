// Local Imports
mod args; // Contains the structure in which are arguments are to be parsed

// Function Imports

use args::{CommandType, SpellChecker};
use clap::{Error, Parser};

fn main() {
    let parsed = SpellChecker::parse();
    match parsed.command_type {
        CommandType::Check(some_arguments) => {
            match spell_checker_function(some_arguments.some_func) {
                Ok(_) => (),
                Err(e) => eprint!("Some Error occured: {:?}", e),
            }
        }
    }
}

fn spell_checker_function(some_function: bool) -> Result<(), Error> {
    //Make your function here
    /*
    Note:
       This project is an ML project since you have to use probabilistic models and dicitonary datasets. Along with n-grams and other ML things.

    Prerequisites:

    Algorithms:
    1. Levenshtein Algorithms [Minimum operations required to convert one word to another word]

    Datasets:
    1. Dictionary Datasets [American English]  //Expand to more datasets after one works

    */
    if some_function {
        println!("This is unecesarry");
    }
    Ok(())
}
