#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

mod dictionary;
mod grid;

fn main() {

    let grid = grid::Grid::new();

    println!("{}", grid)
}