use std::collections::HashMap;
use std::vec::Vec;


mod wordlist;

fn main() {

    let mut frequencies = HashMap::new();

    for word in wordlist::WORDLIST.lines() {
        let index = word.len();

        if ! frequencies.contains_key(&index) {
            frequencies.insert(index, 1);
        } else {
            let optional_count = frequencies.get(&index);
            match optional_count {
                Some(old_value) => frequencies.insert(index, old_value + 1),
                None => panic!("I need to get better at writing Rust."),
            };
        }
    }

    let mut keys = Vec::from_iter(frequencies.keys());

    keys.sort();


    for key in keys {
        println!("{0}: {1}", key, frequencies[key]);

    }
}
