#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
use clap::Parser;

mod dictionary;
mod game;

#[derive(Debug)]
#[derive(Parser)]
struct Cli {
    #[arg(num_args(0..))]
    values: Vec<String>,
}

fn main() {

    let args = Cli::parse();

    let grid;

    if args.values.len() > 0 {
        let mut values = Vec::new();
        for value in args.values {
            values.push(value);
        }
        match game::new_from_values(values) {
            Err(err) => { eprintln!("Error: {}", err); },
            Ok(grid) => { println!("{}", grid); }
        }
    } else {
        grid = game::new_random();
        println!("{}", grid)
    }    
}