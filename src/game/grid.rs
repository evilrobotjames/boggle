use core::fmt;
use rand::Rng;
use rand::prelude::SliceRandom;
use crate::game::dice;

const NUM_ROWS: usize = 4;
const NUM_COLS: usize = 4;
const LEN_ROW: usize = NUM_COLS;
const LEN_COL: usize = NUM_ROWS;

const NUM_CELLS: usize = NUM_ROWS * NUM_COLS;

pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
pub struct Cell <'a> {
    traversed_already: bool,
    value: &'a str,
}

impl <'a> Cell<'a> {
    pub fn new (value: &'a str) -> Self { 
        Self {
            traversed_already: false,
            value: value,
        }
    }
}

#[derive(Debug)]
pub struct Grid <'a> {
    pub cells: Vec<Cell<'a>>,
}

impl <'a> Grid <'_> {

    pub fn new() -> Self {
    
        // Create a new grid, with some random stuff in cells.

        let mut dice_order: Vec<usize> = (0..NUM_CELLS).collect();
        dice_order.shuffle(&mut rand::thread_rng());

        let mut cells = Vec::new();

        for dice_index in dice_order {
            let mut rng = rand::thread_rng();
            let side_index = rng.gen_range(0..dice::NUM_SIDES);

            cells.push(Cell::new(dice::DICE[dice_index][side_index]));
        }

        Grid {
            cells: cells,
        }
    }

    pub fn go(index: usize, direction: Direction) -> Option<usize> {
        match direction {
            Direction::North => if index < LEN_ROW { None } else { Some(index - LEN_ROW) },
            Direction::East => if index % LEN_ROW == 0 { None } else { Some(index + 1) },
            Direction::South => None,
            Direction::West => None,
        }
    }
}

impl fmt::Display for Grid<'_> {
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