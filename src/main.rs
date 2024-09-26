// Local Imports
mod algorithms; // Contains all the algorithms that are used
mod args; // Contains the structure in which are arguments are to be parsed
mod spellchecker; // Contains the spell checker function
mod utils; // Contains Data Structure Implimentation

// Function Imports
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
            match spell_dictionary
                .dictionary
                .contains(&word_argument.word_to_check.to_lowercase())
            {
                true => println!("Yes!!, {:?} is a real word", word_argument.word_to_check),
                false => {
                    let mut current_word = String::new();
                    let mut suggestion = Vec::new();
                    spell_dictionary.trie_dfs(
                        &spell_dictionary.dictionary.root,
                        &word_argument.word_to_check.to_lowercase(),
                        &mut current_word,
                        &mut suggestion,
                    );
                    match suggestion.len() {
                        0 => println!("This word does not correlate with any word in English"),
                        _ => println!("Word misspelled...\nMaybe you mean: {:?}", suggestion),
                    }
                }
            }
        }
    }
}
