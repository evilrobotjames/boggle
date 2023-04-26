use crate::words::testlist::WORDLIST_TEST_SEGMENT;

mod words;

fn main() {

    let root = words::root();

    println!("{:#?}", root);

    let words = root.to_words();

    let wordlist = words.join("\n");
    
    println!("{}", wordlist);

    if wordlist == WORDLIST_TEST_SEGMENT {
        println!("yay");
    } else {
        println!("nay");
    }
}
