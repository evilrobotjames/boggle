mod words;

fn main() {

    let root = words::root();

    let words = root.to_words();
    
    for word in words {
        println!("{}", word);
    }
}
