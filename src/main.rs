#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

mod dictionary;
mod game;

fn main() {

    let grid = game::Grid::new();

    println!("{}", grid)
}