use self::grid::Grid;

mod test;
pub mod dice;
pub mod grid;

use std::string::String;

pub fn new_random() -> Grid {
    Grid::new()
}

pub fn new_from_values(values: Vec<&str>) -> Option<Grid> {
    if values.len() != grid::NUM_CELLS {
        return None;
    }

    Some(Grid::new_from_values(values))
}

pub fn solve(_grid: Grid, _lookup: fn(&str) -> bool) -> Vec<String> {
    // Permute grid to generate potential words, using lookup to valiate.
    // Returns a vector of strings found, sorted by length, longest first.

    assert!(false, "Not implemented");

    let words_found = Vec::new();

    words_found
}
