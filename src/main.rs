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
        let f = File::open("data/dictionary.txt")?;
        let lines = io::BufReader::new(f).lines();
        for line in lines {
            set_dictionary.push(line?);
        }
        Ok(Self {
            dictionary: set_dictionary,
        })
    }

    fn suggestinator(&self, checked_word: &str) -> Result<(Vec<(&str, usize)>, usize)> {
        let mut suggestions: Vec<(&str, usize)> = Vec::new();
        let mut word_counter = 0;
        let file = File::open(checked_word)?;
        let lines = io::BufReader::new(file).lines();
        for (index, line) in lines.enumerate() {
            let mut liner = line?;
            liner.retain(|c| !r#"(),".;:'"#.contains(c));
            for word in liner.split_whitespace() {
                word_counter+=1;
                for i in &self.dictionary {
                    let lev_dist = levenshtein(word, i);
                    if lev_dist == 0 {
                        continue;
                    } else if lev_dist == 1 {
                        suggestions.push((i, index));

                    }
                }
            }
        }

        Ok((suggestions, word_counter))
    }
}
fn main() {
    let parsed = CliArgument::parse();
    match parsed.command_type {
        CommandType::Check(checkargument) => {
            let spell_dictionary = SpellChecker::new().unwrap();
            match spell_dictionary.suggestinator(&checkargument.word_to_be) {
                Ok((some_vec, total_words)) => {
                    for (word, index) in some_vec.iter() {
                        println!("Error on line {:?} : {:?}", index, word);
                    }
                    println!("Total words: {:?}", total_words);
                },
                Err(e) => eprintln!("Error Occured: {:?}", e),
            }
        }
    }
}

/*
Prerequisites:

Algorithms [Fuzzy]:
1. Levenshtein Algorithms [Minimum operations required to convert one word to another word] -- Done
2. Damerau-Levenshtein [Better than Levenshtein ???]

Datasets:
1. Dictionary Datasets [American English]  //Expand to more datasets after one works

Data Structures:
1. Trie : To store compressed form of your dataset and also reduces number of iterations
2. Bloom Filters: To efficiently store words with least memory usage


Steps:

1. Make basic structure of the program. [Done]
2. Make a functioning spellchecker with a small dictionary set and only checks one word. [Done]
3. Expand the one word capability to one file capability. [Done]
4. Use Trie Data Structures to compress that large dataset into smaller space [Performace Boost]. [Ongoing]
    a. Create a program to add all the words in the dictionary dataset into the trie data structure.
    b. Tweak levenshtein to make it favourible to search in trie data structures.
5. Use Bloom Filters to store words more efficiently [Save Storage].
6. Add Features:
    a. Functionality which allows you to add words to your dictionary.
    b. Show the total amount of words in a text file.
    c. See if Demarau-Levenshtein algorithm or Bitap algorithm will be better for your use case.
    d. Complete the full API checklist.
*/
