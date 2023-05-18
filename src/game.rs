use crate::dictionary;

use self::grid::{Grid};

mod test;
pub mod dice;
pub mod grid;

use std::string::String;

pub fn new_random() -> Grid {
    Grid::new_random()
}

pub fn new_from_values(values: Vec<String>) -> Result<Grid, String> {

    Grid::new_from_values(values)
}

fn solve_recurse(grid: &mut Grid, lookup: fn(&str) -> bool, start: usize, potential_word: &mut String) {

    if grid.cells[start].traversed_already {
        return;
    }

    potential_word.push_str(grid.cells[start].value.as_str());
    grid.cells[start].traversed_already = true;

    if dictionary::contains_word(&potential_word) {
        grid.words_found.push(potential_word.to_string());
    }

    for direction in grid::DIRECTIONS.iter() {
        match Grid::go(start, *direction) {
            None => continue,
            Some(next) => {
                solve_recurse(grid, lookup, next, potential_word);
            }
        }
    }

    let mut to_remove = grid.cells[start].value.as_str().len();
    
    while to_remove > 0 {
        potential_word.pop();
        to_remove = to_remove - 1;
    }
    
    grid.cells[start].traversed_already = false;
}

pub fn solve(grid: &mut Grid, lookup: fn(&str) -> bool) {
    // Permute grid to generate potential words, using lookup to valiate.
    // Returns a vector of strings found, sorted by length, longest first.

    grid.words_found = Vec::new();
    let mut potential_word = String::new();

    for cell in 0..grid.cells.len() {

        print!(".");
        solve_recurse(grid, lookup, cell, &mut potential_word);
    }

    println!("");
}
