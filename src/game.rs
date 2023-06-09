use crate::dictionary::{self, tree::IsValidWord};
use IsValidWord::{Yes, No, Never};

use self::grid::{Grid};

mod test;
pub mod dice;
pub mod grid;

use std::string::String;

pub fn new_random(side_length: usize) -> Result<Grid, String> {
    Grid::new_random(side_length)
}

pub fn new_from_values(values: Vec<String>) -> Result<Grid, String> {

    Grid::new_from_values(values)
}

fn solve_recurse(grid: &mut Grid, lookup: fn(&str) -> IsValidWord, start: usize, potential_word: &mut String) {

    // Bail if we've already seen this node.
    if grid.cells[start].traversed_already {
        return;
    }

    // Add to potential_word, mark as traversed.
    potential_word.push_str(grid.cells[start].value.as_str());
    grid.cells[start].traversed_already = true;


    // Valid word?  Add it to the list.
    let contains_word = dictionary::contains_word(&potential_word);
    if contains_word == Yes {
        grid.words_found.push(potential_word.to_string());
    }

    // If this sequence of characters is never going to yield a word, stop recursing.
    match contains_word {
        Yes | No => {
            for direction in grid::DIRECTIONS.iter() {
                match grid.go(start, *direction) {
                    None => continue,
                    Some(next) => {
                        solve_recurse(grid, lookup, next, potential_word);
                    }
                }
            }
        },
        Never => ()
    }

    // Remove from potential_word, unmark as traversed.
    let mut to_remove = grid.cells[start].value.as_str().len();

    while to_remove > 0 {
        potential_word.pop();
        to_remove = to_remove - 1;
    }

    grid.cells[start].traversed_already = false;
}

pub fn solve(grid: &mut Grid, lookup: fn(&str) -> IsValidWord) {
    // Permute grid to generate potential words, using lookup to valiate.
    // Returns a vector of strings found, sorted by length, longest first.

    grid.words_found = Vec::new();
    let mut potential_word = String::new();

    for cell in 0..grid.cells.len() {

        solve_recurse(grid, lookup, cell, &mut potential_word);
    }

    println!("");
}
