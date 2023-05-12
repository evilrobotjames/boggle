#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
use crate::game::grid::Grid;

mod dictionary;
mod game;

fn main() {

    let grid = Grid::new();

    println!("{}", grid)
}