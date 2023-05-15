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

    println!("{:?}", args);

    let grid = game::new_random();

    println!("{}", grid)
}