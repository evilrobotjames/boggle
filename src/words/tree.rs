use std::collections::HashMap;
//use std::vec::Vec;

#[derive(Debug)]
pub struct LetterNode {
    pub word: bool,
    pub next: HashMap<char, LetterNode>,
}

impl LetterNode {
    fn new() -> LetterNode {
        LetterNode { word: false, next: HashMap::new() }
    }

    fn insert_string(&mut self, string:&str) {
        match string.chars().next() {
            None => {
                // The string is now empty.  Mark this node as a complete word.
                self.word = true;
            },
            Some(c) => {
                // Ensure next LetterNode exists for the first letter
                let next = self.next.entry(c).or_insert(LetterNode::new());
                // Recurse through the remaining string
                next.insert_string(&string[1..]);
            }
        }
    }

/*
    pub fn to_words(&self) -> Vec<String> {
        let mut words = Vec::new();
        let mut potential_word = String::new();

        fn recurse(node: &LetterNode) {
            for letter_to_letternode in node.next {
                if node.word {
                    // this is a complete word.  add current potential_word to words vector and continue.
                }
                let letter = letter_to_letternode.0;
                let letternode = letter_to_letternode.1;

                potential_word.push(letter);
                recurse(&letternode);
            }
        }

        recurse(self);

        words
    }
*/
}

pub fn generate_root(wordlist: &str) -> LetterNode {

    let mut root = LetterNode::new();

    for word in wordlist.lines() {
        root.insert_string(&word);
    }

    root
}