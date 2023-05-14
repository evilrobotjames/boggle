#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
use clap::Parser;

mod dictionary;
mod game;

#[derive(Debug)]
#[derive(Parser)]
struct Cli {
}

fn main() {

    let _args = Cli::parse();

    let grid = game::new_random();

    println!("{}", grid)
}