use std::collections::HashMap;
use std::string::String;

#[derive(Debug)]
pub struct LetterNode {
    pub characters: String,
    pub next: HashMap<String, LetterNode>,
}

pub fn get_root() -> LetterNode
{
    LetterNode {
        characters: String::new(),
        next: HashMap::new()
    }
}

