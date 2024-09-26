// Local Imports
mod algorithms; // Contains all the algorithms that are used
mod args; // Contains the structure in which are arguments are to be parsed
mod spellchecker; // Contains the spell checker function
mod utils; // Contains Data Structure Implimentation

use args::{CliArgument, CommandType};
use clap::Parser;
use spellchecker::SpellChecker;

fn main() {
    let parsed = CliArgument::parse();
    let spell_dictionary = SpellChecker::new().unwrap();
    match parsed.command_type {
        CommandType::Check(checkargument) => {
            match spell_dictionary.suggestinator(&checkargument.word_to_be) {
                Ok((some_hash, total_words)) => {
                    for (suggestions, line_no) in some_hash {
                        if suggestions.len() > 1 {
                            let wrong = &suggestions[0];
                            let mut new_list = suggestions.clone();
                            new_list.remove(0);
                            println!(
                                "Line: {:?} - Error: {:?} : Suggestions : {:?}",
                                line_no, wrong, new_list
                            );
                        }
                    }
                    println!("Total words: {:?}", total_words);
                }
                Err(e) => eprintln!("Error Occured: {:?}", e),
            }
        }
        CommandType::Is(word_argument) => {
            if spell_dictionary
                .dictionary
                .contains(&word_argument.word_to_check)
            {
                println!("Yes!!, {:?} is a real word", word_argument.word_to_check);
            } else {
                println!("Word doesn't exist");
            }
        }
    }
}
