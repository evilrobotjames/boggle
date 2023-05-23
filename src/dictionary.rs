use self::tree::Node;

mod test;
pub mod list;
pub mod tree;
pub mod testlist;


lazy_static! {
    static ref ROOT: Node = tree::generate_root(list::WORDLIST);
}

pub fn to_list() -> String {

    ROOT.to_list()
}

pub fn contains_word(word: &str) -> tree::IsValidWord {

    ROOT.contains_word(word)
}