use self::tree::Node;

mod test;
pub mod list;
pub mod tree;
pub mod testlist;

pub fn root() -> Node {
    tree::generate_root(list::WORDLIST)
}
