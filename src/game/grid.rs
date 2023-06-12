use core::fmt;
use std::str::FromStr;
use rand::prelude::SliceRandom;
use crate::game::dice::{self, DICE_SETS, DiceSet};

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub static DIRECTIONS: [Direction; 8] = [
    Direction::North, Direction::NorthEast, Direction::East, Direction::SouthEast,
    Direction::South, Direction::SouthWest, Direction::West, Direction::NorthWest];


#[derive(Debug)]
pub struct Cell {
    pub traversed_already: bool,
    pub value: String,
}

impl Cell {
    pub fn new(value: &str) -> Self { 

        let value = String::from_str(value).expect("String should be parsable.");

        Self {
            traversed_already: false,
            value,
        }
    }
}

#[derive(Debug)]
pub struct Grid {
    pub side_length: usize,
    pub cells: Vec<Cell>,
    pub words_found: Vec<String>,
}

pub fn side_length_from_cell_count(cell_count: usize) -> Option<usize> {
    let cell_count= cell_count as f64;
    let side_length = cell_count.sqrt();
    if side_length.fract() == 0.0 {
        Some(side_length as usize)
    } else {
        None
    }
}

pub fn invalid_cell_count_diagnostic(cell_count: usize) -> String {
    // Dog's breakfast.  Rewrite with iter.find().
    let diagnostic = format!("Cells specified: {cell_count}.  Require: ");
    let mut diagnostic = String::from_str(&diagnostic).unwrap();
    let mut required = String::new();
    let mut first = true;
    for dice_set in DICE_SETS.iter() {
        if !first {
            required.push_str(", ");
        }
        first = false;
        let len = dice_set.len();
        required.push_str(format!("{len}").as_str());
    }
    diagnostic.push_str(required.as_str());
    diagnostic
}



pub fn lookup_dice_set_by_cell_count(cell_count: usize) -> Result<&'static DiceSet, String> {
    for dice_set in DICE_SETS.iter() {
        let len = dice_set.len();
        if len == cell_count {
            return Ok(dice_set)
        }
    }

    Err(invalid_cell_count_diagnostic(cell_count))
}

pub fn lookup_dice_set_by_side_length(side_length: usize) -> Result<&'static DiceSet, String> {
    // Let's have an explicit one of these.  I've passed in a side_length as a 
    // cell_count more than once.
    lookup_dice_set_by_cell_count(side_length * side_length)
}


impl Grid {

    pub fn new_random(side_length: usize) -> Result<Self, String> {

        let cell_count = side_length * side_length;
        let dice_set = lookup_dice_set_by_side_length(side_length)?;

        // Create a new grid, with some random stuff in cells.

        let mut dice_sequence: Vec<usize> = (0..cell_count).collect();
        dice_sequence.shuffle(&mut rand::thread_rng());

        let mut cells = Vec::new();

        for dice_index in dice_sequence {
            let value = dice::roll(dice_set[dice_index]);
            cells.push(Cell::new(value));
        }

        Ok(Grid {
            side_length,
            cells,
            words_found: Vec::new(),        
        })
    }

    pub fn new_from_values<S>(values: Vec<S>) -> Result<Self, String> where S: Into<String>{

        let dice_count = values.len();
        let _dice_set = lookup_dice_set_by_cell_count(dice_count)?;

        // Construct the Grid.

        let mut cells = Vec::new();

        for value in values {
            let value = value.into();
            if !value.chars().all(char::is_alphabetic) {
                return Err(format!("'{value}' not alpha"));
            }
            cells.push(Cell::new(value.as_str()));
        }

        let side_length = side_length_from_cell_count(cells.len()).expect("Should have a valid side_length, given we've already found a dice set.");

        Ok(Grid { side_length, words_found: Vec::new(), cells })
    }

    pub fn words_found(&mut self) -> &Vec<String> {
        self.words_found.sort_by(|a, b| b.len().cmp(&a.len()));
        &self.words_found
    }

    pub fn is_north_edge(&self, index: usize) -> bool {
        index < self.side_length
    }

    pub fn is_east_edge(&self, index: usize) -> bool {
        index % self.side_length >= self.side_length - 1
    }

    pub fn is_south_edge(&self, index: usize) -> bool {
        index >= self.cells.len() - self.side_length
    }
    
    pub fn is_west_edge(&self, index: usize) -> bool {
        index % self.side_length == 0
    }

    pub fn go(&self, index: usize, direction: Direction) -> Option<usize> {
        // Return the index of the cell to the direction of the cell specified by index
        match direction {
            Direction::North => if self.is_north_edge(index) { None } else { Some(index - self.side_length) },
            Direction::NorthEast => if self.is_east_edge(index) || self.is_north_edge(index) { None } else { Some(index - self.side_length + 1) },
            Direction::East => if self.is_east_edge(index) { None } else { Some(index + 1) },
            Direction::SouthEast => if self.is_east_edge(index) || self.is_south_edge(index) { None } else { Some(index + self.side_length + 1) },
            Direction::South => if self.is_south_edge(index) { None } else { Some(index + self.side_length) }, 
            Direction::SouthWest => if self.is_west_edge(index) || self.is_south_edge(index) { None } else { Some(index + self.side_length - 1) },
            Direction::West => if self.is_west_edge(index) { None } else { Some(index - 1) },
            Direction::NorthWest => if self.is_west_edge(index) || self.is_north_edge(index) { None } else { Some(index - self.side_length - 1) },
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut output = String::new();

        for row in 0..self.side_length {

            if row > 0 {
                output.push('\n');
            }

            for col in 0..self.side_length {
                output.push_str(format!("{} ", self.cells[self.side_length * row + col].value).as_str());
            }            
        }

        write!(f, "{}", output)
    }
}