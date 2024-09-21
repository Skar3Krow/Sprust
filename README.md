# Sprust
## Project Overview
This project is a Rust-based spell checker that uses a Trie (Prefix Tree) data structure to store and efficiently search through a dictionary of words. The spell checker can:
1. Check a file for spellig errors.
2. Suggest corrections for misspelled words using the edit distance (Levenshtein distance) algorithm.
3. Provide word suggestions based on common prefixes.

## 1. Installation
### Pre-requisites:
1. Ensure that Rust is installed on your system. You can install Rust using the following command:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Cloning the Repository:
```
git clone https://github.com/Skar3Krow/Sprust.git
cd Sprust
```

4. Building the Project:
```
cargo install --path .
```

5. Running the Tools:
Once built, you can run the tools using the following command:
```
sprust check <filepath>
```

## 2. Trie Data Structure

A Trie is a specialized tree used for storing strings where each node represents a single character. The Trie allows for:

- Efficient search of words: O(m) time complexity, where m is the length of the word.
- Finding all words that share a common prefix (useful for autocompletion or suggestions).

### Trie Node Structure
```rust
#[derive(Debug, Default)]
pub struct TrieNode {
  pub child: HashMap<char, TrieNode>,
  pub is_end_of_word: bool,
}
```
Each TrieNode contains:

children: A HashMap that maps each character to another TrieNode.
is_end_of_word: A boolean flag to indicate the end of a word in the Trie.
```rust
#[derive(Debug, Default)]
pub struct Trie {
  pub root: TrieNode,
}
```
The Trie has the following methods:

- ```new()```: Initializes an empty Trie.
- ```insert(word: &str)```: Inserts a word into the Trie.
- ```contains(word: &str)```: Searches for an exact word in the Trie.

## 3. Spell Checker Implementation
The SpellChecker struct builds on the Trie data structure and extends it with spell-checking functionality.
### SpellChecker Struct
```rust
pub struct SpellChecker {
  pub dictionary: Trie,
}
```
The SpellChecker has the following functionalities:
### Check
```
sprust check <filepath>
```
- Description: Finds out the incorrect word in given file and suggest correct words.
- Parameters:
- - file_path: The file path of the file to be spell checked
- Return: Spelling mistakes and their suggestions.

## 4. Example Usage
Here is how you can use the SpellChecker:
```
sprust check data/spell_error.txt
```
#### Expected Output
```
Line: 1 - Error: "nzim" : Suggestions : ["nazim", "naim", "nim"]
Line: 2 - Error: "anithing" : Suggestions : ["nithing", "anything"]
Line: 0 - Error: "helt" : Suggestions : ["telt", "celt", "elt", "felt", "kelt", "pelt", "welt", "melt", "belt", "yelt", "gelt", "hest", "hel", "hell", "held", "helot", "helm", "help", "hele", "het", "hert", "hewt", "heft", "heat", "hent", "hilt", "halt", "holt", "selt"]
Line: 0 - Error: "somethin" : Suggestions : ["something"]
Line: 0 - Error: "nohting" : Suggestions : ["noting"]
Line: 2 - Error: "rustup" : Suggestions : ["dustup"]
Line: 0 - Error: "henlo" : Suggestions : ["hello"]
Total words: 38
```

## 5. Future Improvements
- Addition Heuristics: More advanced heuristics for word correction, such as phonetic algorithms such as Soundex, could improve the quality of suggestions.
- Word Existence: This can be extended to support word existence by leveraging the ```contians``` method.
- Autocompletion: The trie structure can be extended to support autocompletion.

## 6. Conclusion
This project demonstrates how to efficiently implement a spell checker using a Trie in Rust. By utilizing a Trie for fast word lookups and combining it with edit distance calculation, the spell checker can correct and suggest words even for moderately misspelled inputs. This approach is scalable and can be further extended to handle large dictionaries and more complex correction algorithms.
