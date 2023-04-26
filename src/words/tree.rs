use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
pub struct Node {
    pub word: bool,
    pub next: HashMap<char, Node>,
}

impl Node {
    fn new() -> Node {
        Node { word: false, next: HashMap::new() }
    }

    fn insert_string(&mut self, string:&str) {
        match string.chars().next() {
            None => {
                // The string is now empty.  Mark this node as a complete word.
                self.word = true;
            },
            Some(c) => {
                // Ensure next LetterNode exists for the first letter
                let next = self.next.entry(c).or_insert(Node::new());
                // Recurse through the remaining string
                next.insert_string(&string[1..]);
            }
        }
    }

    fn to_words_recurse(&self, words: &mut Vec<String>, potential_word: &mut String) {
        for letter_to_node in self.next.iter() {
            let letter = letter_to_node.0;
            let node = letter_to_node.1;

            potential_word.push(*letter);

            if node.word {
                // this is a complete word.  add current potential_word to words vector and continue.
                let confirmed_word = String::from(potential_word.as_str());
                // TODO: Do we keep this list sorted in the first place?  Or sort it all at the end?
                words.push(confirmed_word);
            }

            node.to_words_recurse(words, potential_word);
            potential_word.pop();
        }
    }

    pub fn to_words(&self) -> Vec<String> {
        let mut words: Vec<String> = Vec::new();
        let mut potential_word = String::new();

        self.to_words_recurse(&mut words, &mut potential_word);

        words.sort();
        words
    }
}

pub fn generate_root(wordlist: &str) -> Node {

    let mut root = Node::new();

    for word in wordlist.lines() {
        root.insert_string(&word);
    }

    root
}