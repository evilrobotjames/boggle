mod wordlist;

fn main() {

    let mut maxlen = 0;

    for line in wordlist::WORDLIST.lines() {
        let len = line.len();
        if len > maxlen {
            maxlen = len;
            println!("maxlen: {maxlen}, \"{line}\"");
        }
    }
}
