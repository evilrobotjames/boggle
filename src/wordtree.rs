use std::collections::HashMap;

use crate::wordlist;

#[derive(Debug)]
pub struct LetterNode {
    pub word: bool,
    pub next: HashMap<char, LetterNode>,
}

pub fn generate_root() -> LetterNode {
    let mut root = LetterNode {
        word: false,
        next: HashMap::new()
    };

    for word in wordlist::WORDLIST_TEST.lines() {
        insert_string(&mut root, &word);
    }

    root
}

fn insert_string(node: &mut LetterNode, string: &str) {

    match string.chars().next() {
        None => {
            // The string is now empty.  Mark this node as a complete word.
            node.word = true;
        },
        Some(c) => {
            println!("{}", c);
            // ensure next LetterNode exists for the first letter, and recurse with the remaining string.
            // insert_string(next_node, sequence[1:])
            node.next.entry(c).or_insert(LetterNode { word: false, next: HashMap::new() });

        }
    }
}
