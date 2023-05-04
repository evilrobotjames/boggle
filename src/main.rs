#[macro_use]
extern crate lazy_static;

mod words;

fn main() {

    println!("yay: {}", words::contains_word("smile"));

    println!("{}", words::to_list());  
}