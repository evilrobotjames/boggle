use self::grid::Grid;

mod test;
pub mod dice;
pub mod grid;

use std::string::String;

pub fn new_random(_size: usize) {
    // Create new grid with random values.
}

pub fn new_from_values(size: usize, values: Vec<&str>) -> Option<Grid> {
    if values.len() != size * size {
        return None;
    }

    None
    // Create new grid from values.
}

pub fn solve(_grid: Grid, _lookup: fn(&str) -> bool) -> Vec<String> {
    // Permute grid to generate potential words, using lookup to valiate.
    // Returns a vector of strings found, sorted by length, longest first.

    let words_found = Vec::new();

    words_found
}
