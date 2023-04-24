use std::collections::HashMap;

#[derive(Debug)]
pub struct LetterNode {
    pub word: bool,
    pub next: HashMap<char, LetterNode>,
}

impl LetterNode {
    fn new() -> LetterNode {
        LetterNode { word: false, next: HashMap::new() }
    }
}

pub fn generate_root(wordlist: &str) -> LetterNode {

    let mut root = LetterNode::new();

    for word in wordlist.lines() {
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
            // Ensure next LetterNode exists for the first letter
            let next = node.next.entry(c).or_insert(LetterNode::new());
            // Recurse through the remaining string
            insert_string(next, &string[1..]);
        }
    }
}