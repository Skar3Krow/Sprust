// Local Imports
mod algorithms; // Contains all the algorithms that are used
mod args; // Contains the structure in which are arguments are to be parsed

// Funciton Imports
use algorithms::levenshtein;
use args::{CliArgument, CommandType};
use clap::Parser;
use core::str;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Result};

// Trie Node defined here [Move to new file later]
#[derive(Default, Debug)]
struct TrieNode {
    pub is_end_of_word: bool,
    pub child: HashMap<char, TrieNode>,
}

#[derive(Debug, Default)]
struct Trie {
    pub root: TrieNode,
}

#[allow(dead_code)]
impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    fn insert(&mut self, word: &str) {
        let mut cur_node = &mut self.root;
        for c in word.chars() {
            cur_node = cur_node.child.entry(c).or_default();
        }
        cur_node.is_end_of_word = true;
    }

    fn contains(&self, word: &str) -> bool {
        let mut cur_node = &self.root;
        for i in word.chars() {
            match cur_node.child.get(&i) {
                Some(node) => cur_node = node,
                None => return false,
            }
        }
        cur_node.is_end_of_word
    }
}

struct SpellChecker {
    /// File name with the words that are writtens
    dictionary: Trie,
}

impl SpellChecker {
    fn new() -> Result<Self> {
        let mut set_dictionary: Trie = Trie::new();
        let f = File::open("data/words_alpha.txt")?;
        let lines = io::BufReader::new(f).lines();
        for line in lines {
            set_dictionary.insert(&line?);
        }
        Ok(Self {
            dictionary: set_dictionary,
        })
    }

    fn trie_dfs(
        &self,
        node: &TrieNode,
        target_word: &str,
        current_word: &mut String,
        suggestions: &mut Vec<String>,
        line_number: &usize,
    ) {
        if node.is_end_of_word && levenshtein(&current_word, &target_word) == 1 {
            suggestions.push(current_word.clone());
        }
        for (&ch, next_node) in &node.child {
            current_word.push(ch);
            self.trie_dfs(
                next_node,
                target_word,
                current_word,
                suggestions,
                line_number,
            );
            current_word.pop();
        }
    }

    fn suggestinator(&self, checked_word: &str) -> Result<(HashMap<Vec<String>, usize>, usize)> {
        let mut word_counter = 0;
        let mut block: HashMap<Vec<String>, usize> = HashMap::new();
        let file = File::open(checked_word)?;
        let lines = io::BufReader::new(file).lines();
        for (index, line) in lines.enumerate() {
            let mut liner = line?;
            liner.retain(|c| !r#"(),".;:'"#.contains(c));
            for word in liner.split_whitespace() {
                word_counter += 1;
                if self.dictionary.contains(&word) {
                    continue;
                }
                let mut current_word = String::new();
                let mut suggestions: Vec<String> = Vec::new();
                suggestions.push(word.to_string());
                self.trie_dfs(
                    &self.dictionary.root,
                    &word,
                    &mut current_word,
                    &mut suggestions,
                    &index,
                );
                block.insert(suggestions.clone(), index);
            }
        }

        Ok((block, word_counter))
    }
}
fn main() {
    let parsed = CliArgument::parse();
    match parsed.command_type {
        CommandType::Check(checkargument) => {
            let spell_dictionary = SpellChecker::new().unwrap();
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
    }
}
