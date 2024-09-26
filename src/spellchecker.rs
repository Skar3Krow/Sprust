// Local Imports
use crate::algorithms::damerau_levenshtein;
use crate::utils::{Trie, TrieNode};

// Functional Imports
use core::str;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Result};

pub struct SpellChecker {
    /// File name with the words that are writtens
    pub dictionary: Trie,
}

impl SpellChecker {
    pub fn new() -> Result<Self> {
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

    pub fn trie_dfs(
        &self,
        node: &TrieNode,
        target_word: &str,
        current_word: &mut String,
        suggestions: &mut Vec<String>,
    ) {
        if node.is_end_of_word && damerau_levenshtein(&current_word, &target_word) == 1 {
            suggestions.push(current_word.clone());
        }
        for (&ch, next_node) in &node.child {
            current_word.push(ch);
            self.trie_dfs(next_node, target_word, current_word, suggestions);
            current_word.pop();
        }
    }

    pub fn suggestinator(
        &self,
        checked_word: &str,
    ) -> Result<(HashMap<Vec<String>, usize>, usize)> {
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
                    &word.to_lowercase(),
                    &mut current_word,
                    &mut suggestions,
                );
                block.insert(suggestions.clone(), index);
            }
        }

        Ok((block, word_counter))
    }
}
