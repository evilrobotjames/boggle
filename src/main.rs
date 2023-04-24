mod wordlist;
mod wordtree;

fn main() {

    let root = wordtree::generate_root(wordlist::WORDLIST_TEST2);

    println!("{:#?}", root);
}
