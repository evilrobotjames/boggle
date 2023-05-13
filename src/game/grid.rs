use core::fmt;
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
            value,
        }
    }
}

#[derive(Debug)]
pub struct Grid <'a> {
    cells: Vec<Cell<'a>>,
}

impl <'a> Grid <'_> {

    pub fn new() -> Self {
    
        // Create a new grid, with some random stuff in cells.

        let mut dice_order: Vec<usize> = (0..NUM_CELLS).collect();
        dice_order.shuffle(&mut rand::thread_rng());

        let mut cells = Vec::new();

        for dice_index in dice_order {
            let value = dice::roll(dice::DICE[dice_index]);
            cells.push(Cell::new(value));
        }

        Grid {
            cells: cells,
        }
    }

    pub fn go(index: usize, direction: Direction) -> Option<usize> {
        // Return the index of the cell to the direction of the cell specified by index
        match direction {
            Direction::North => if index < LEN_ROW { None } else { Some(index - LEN_ROW) },
            Direction::East => if index % LEN_ROW == 0 { None } else { Some(index + 1) },
            Direction::South => if index + LEN_ROW > NUM_CELLS { None } else { Some(index + LEN_ROW) }, 
            Direction::West => if index % LEN_ROW == 3 { None } else { Some(index - 1) },
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