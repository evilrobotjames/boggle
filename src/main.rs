#[macro_use]
extern crate lazy_static;

mod dictionary;

fn main() {

    println!("Is smile a word?:  {}", dictionary::contains_word("smile"));

    println!("{}", dictionary::to_list());  
}