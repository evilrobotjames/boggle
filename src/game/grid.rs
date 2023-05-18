use core::fmt;
use std::str::FromStr;
use rand::prelude::SliceRandom;
use crate::game::dice;

const SIZE: usize = 4;
const NUM_ROWS: usize = SIZE;
const NUM_COLS: usize = SIZE;
const LEN_ROW: usize = NUM_COLS;
const LEN_COL: usize = NUM_ROWS;

pub const NUM_CELLS: usize = NUM_ROWS * NUM_COLS;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub static DIRECTIONS: [Direction; 4] = [Direction::North, Direction::East, 
                                    Direction::South, Direction::West];

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
    pub words_found: Vec<String>,
    pub cells: Vec<Cell>,
}

impl Grid {

    pub fn new_random() -> Self {

        // Create a new grid, with some random stuff in cells.

        let mut dice_order: Vec<usize> = (0..NUM_CELLS).collect();
        dice_order.shuffle(&mut rand::thread_rng());

        let mut cells = Vec::new();

        for dice_index in dice_order {
            let value = dice::roll(dice::DICE[dice_index]);
            cells.push(Cell::new(value));
        }

        Grid {
            words_found: Vec::new(),
            cells: cells,
        }
    }

    pub fn new_from_values(values: Vec<String>) -> Result<Self, String> {

        let mut cells = Vec::new();

        if values.len() != NUM_CELLS {
            return Err(format!("Found {} values, require {NUM_CELLS}", values.len()));
        }

        for value in values {
            if !value.chars().all(char::is_alphabetic) {
                return Err(format!("'{value}' not alpha"));
            }
            cells.push(Cell::new(value.as_str()));
        }

        let grid = Grid { words_found: Vec::new(), cells };

        Ok(grid)
    }

    pub fn go(index: usize, direction: Direction) -> Option<usize> {
        // Return the index of the cell to the direction of the cell specified by index
        match direction {
            Direction::North => if index < LEN_ROW { None } else { Some(index - LEN_ROW) },
            Direction::East => if index % LEN_ROW == (LEN_ROW - 1) { None } else { Some(index + 1) },
            Direction::South => if index + LEN_ROW >= NUM_CELLS { None } else { Some(index + LEN_ROW) }, 
            Direction::West => if index % LEN_ROW == 0 { None } else { Some(index - 1) },
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        assert!(self.cells.len() == NUM_COLS * NUM_ROWS);

        let mut output = String::new();

        for row in 0..NUM_ROWS {

            if row > 0 {
                output.push('\n');
            }

            for col in 0..NUM_COLS {
                output.push_str(format!("{} ", self.cells[NUM_COLS * row + col].value).as_str());
            }            
        }

        write!(f, "{}", output)
    }
}