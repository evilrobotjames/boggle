mod wordlist;
mod wordtree;

fn main() {

    let root = wordtree::generate_root();

    println!("{:?}", root);
}
