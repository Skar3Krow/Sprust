//Functional Imports
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct TrieNode {
    pub is_end_of_word: bool,
    pub child: HashMap<char, TrieNode>,
}

#[derive(Debug, Default)]
pub struct Trie {
    pub root: TrieNode,
}

#[allow(dead_code)]
impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut cur_node = &mut self.root;
        for c in word.chars() {
            cur_node = cur_node.child.entry(c).or_default();
        }
        cur_node.is_end_of_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
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
