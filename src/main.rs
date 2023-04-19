use std::collections::HashMap;
use std::vec::Vec;

mod wordlist;
mod wordtree;

fn main() {

    let mut frequencies = HashMap::new();

    for word in wordlist::WORDLIST.lines() {
        let key = word.len();
        frequencies.entry(key).and_modify(|val| *val += 1).or_insert(1);
    }

    let mut keys = Vec::from_iter(frequencies.keys());
    keys.sort();

    for key in keys {
        println!("{0}: {1}", key, frequencies[key]);
    }

    let root = wordtree::get_root();

    println!("{:?}", root);
}
