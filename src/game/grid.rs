use core::fmt;
use std::str::FromStr;
use rand::prelude::SliceRandom;
use crate::game::dice;

pub const SIZES_SUPPORTED: [usize; 1] = [4];

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
    pub size: usize,
    pub cells: Vec<Cell>,
    pub words_found: Vec<String>,
}

impl Grid {

    pub fn new_random(size: usize) -> Self {

        assert!(SIZES_SUPPORTED.contains(&size));

        // Create a new grid, with some random stuff in cells.

        let mut dice_order: Vec<usize> = (0..size*size).collect();
        dice_order.shuffle(&mut rand::thread_rng());

        let mut cells = Vec::new();

        for dice_index in dice_order {
            let value = dice::roll(dice::DICE[dice_index]);
            cells.push(Cell::new(value));
        }

        Grid {
            size,
            cells: cells,
            words_found: Vec::new(),        }
    }

    pub fn new_from_values<S>(values: Vec<S>) -> Result<Self, String> where S: Into<String>{

        let mut size = 0;
        for s in SIZES_SUPPORTED {
            if values.len() == s*s {
                size = s
            }
        }

        if 0 == size {
            return Err(format!("Found {} values, require {:?}", values.len(), SIZES_SUPPORTED));
        }

        // Construct the Grid.

        let mut cells = Vec::new();

        for value in values {
            let value = value.into();
            if !value.chars().all(char::is_alphabetic) {
                return Err(format!("'{value}' not alpha"));
            }
            cells.push(Cell::new(value.as_str()));
        }

        Ok(Grid { size, words_found: Vec::new(), cells })
    }

    pub fn words_found(&mut self) -> &Vec<String> {
        self.words_found.sort_by(|a, b| b.len().cmp(&a.len()));
        &self.words_found
    }

    pub fn is_north_edge(&self, index: usize) -> bool {
        index < self.size
    }

    pub fn is_east_edge(&self, index: usize) -> bool {
        index % self.size >= self.size - 1
    }

    pub fn is_south_edge(&self, index: usize) -> bool {
        index >= self.size.pow(2) - self.size
    }
    
    pub fn is_west_edge(&self, index: usize) -> bool {
        index % self.size == 0
    }

    pub fn go(&self, index: usize, direction: Direction) -> Option<usize> {
        // Return the index of the cell to the direction of the cell specified by index
        match direction {
            Direction::North => if self.is_north_edge(index) { None } else { Some(index - self.size) },
            Direction::NorthEast => if self.is_east_edge(index) || self.is_north_edge(index) { None } else { Some(index - self.size + 1) },
            Direction::East => if self.is_east_edge(index) { None } else { Some(index + 1) },
            Direction::SouthEast => if self.is_east_edge(index) || self.is_south_edge(index) { None } else { Some(index + self.size + 1) },
            Direction::South => if self.is_south_edge(index) { None } else { Some(index + self.size) }, 
            Direction::SouthWest => if self.is_west_edge(index) || self.is_south_edge(index) { None } else { Some(index + self.size - 1) },
            Direction::West => if self.is_west_edge(index) { None } else { Some(index - 1) },
            Direction::NorthWest => if self.is_west_edge(index) || self.is_north_edge(index) { None } else { Some(index - self.size - 1) },
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut output = String::new();

        for row in 0..self.size {

            if row > 0 {
                output.push('\n');
            }

            for col in 0..self.size {
                output.push_str(format!("{} ", self.cells[self.size * row + col].value).as_str());
            }            
        }

        write!(f, "{}", output)
    }
}