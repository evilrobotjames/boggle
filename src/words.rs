use self::tree::Node;

pub mod list;
pub mod tree;
pub mod testlist;

pub fn root() -> Node {
    //tree::generate_root(list::WORDLIST)
    tree::generate_root(testlist::WORDLIST_TEST_SEGMENT)
}
