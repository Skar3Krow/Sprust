// Local Imports
mod algorithms; // Contains all the algorithms that are used
mod args; // Contains the structure in which are arguments are to be parsed

use std::fs::File;
use std::io::{self, BufRead, Result};

// Function Imports
use algorithms::levenshtein;
use args::{CliArgument, CommandType};
use clap::Parser;

struct SpellChecker {
    /// File name with the words that are writtens
    dictionary: Vec<String>,
}

impl SpellChecker {
    fn new() -> Result<Self> {
        let mut set_dictionary: Vec<String> = Vec::new();
        let f = File::open("/Users/skar3krow/Desktop/Projects/spell_checker/data/dictionary.txt")?;
        let lines = io::BufReader::new(f).lines();
        for line in lines {
            set_dictionary.push(line?);
        }
        Ok(Self {
            dictionary: set_dictionary,
        })
    }

    fn suggestinator(&self, checked_word: &str) -> Result<Vec<&str>> {
        let mut suggestions: Vec<&str> = Vec::new();
        for i in &self.dictionary {
            if levenshtein(checked_word, i) <= 1 {
                suggestions.push(i);
            }
        }
        Ok(suggestions)
    }
}
fn main() {
    let parsed = CliArgument::parse();
    match parsed.command_type {
        CommandType::Check(checkargument) => {
            let spell_dictionary = SpellChecker::new().unwrap();
            match spell_dictionary.suggestinator(&checkargument.word_to_be) {
                Ok(some_vec) => println!("Suggested words: {:?}", some_vec),
                Err(e) => eprintln!("Error Occured: {:?}", e),
            }
        }
    }
}
//Make your function here
/*
Note:
   This project is an ML project since you have to use probabilistic models and dicitonary datasets. Along with n-grams and other ML things.

Prerequisites:

Algorithms [Fuzzy]:
1. Levenshtein Algorithms [Minimum operations required to convert one word to another word] -- Done
2. Damerau-Levenshtein [Better Levenshtein ???]

Datasets:
1. Dictionary Datasets [American English]  //Expand to more datasets after one works

Data Structures:
1. Trie : To store compressed form of your dataset and also reduces number of iterations
2. Bloom Filters: To efficiently store words with least memory usage

*/
