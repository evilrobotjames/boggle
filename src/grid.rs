use core::fmt;

use rand::{Rng, seq::SliceRandom};

const NUM_DICE: usize = 16;
const NUM_SIDES: usize = 6;
const NUM_CELLS: usize = 16;
const NUM_ROWS: usize = 4;
const NUM_COLS: usize = 4;

static DICE: [[&str; NUM_SIDES]; NUM_DICE] = [
    ["a", "a", "e", "e", "g", "n"],
    ["a", "b", "b", "j", "o", "o"],
    ["a", "c", "h", "o", "p", "s"],
    ["a", "f", "f", "k", "p", "s"],
    ["a", "o", "o", "t", "t", "w"],
    ["h", "l", "n", "n", "r", "z"],
    ["c", "i", "m", "o", "t", "u"], 
    ["d", "e", "i", "l", "r", "x"],
    ["d", "e", "l", "r", "v", "y"],
    ["d", "i", "s", "t", "t", "y"],
    ["e", "e", "g", "h", "n", "w"],
    ["e", "e", "i", "n", "s", "u"],
    ["e", "h", "r", "t", "v", "w"],
    ["e", "i", "o", "s", "s", "t"],
    ["e", "l", "r", "t", "t", "y"],
    ["h", "i", "m", "n", "qu", "u"],
];

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
    cells: [Cell<'a>; NUM_CELLS],
}

impl <'a> Grid <'_> {

    pub fn new() -> Grid <'a> {
    
        // Create a new grid, with some random stuff in cells.

        let mut dice_order: Vec<usize> = (0..NUM_CELLS).collect();
        dice_order.shuffle(&mut rand::thread_rng());

        let mut cells = Vec::new();

        for dice_index in dice_order {
            let mut rng = rand::thread_rng();
            let side_index = rng.gen_range(0..NUM_SIDES);

            cells.push(Cell::new(DICE[dice_index][side_index]));
        }

        Grid {
            cells: [
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
                cells.pop().expect("This has enough values"),
            ]
        }
    }
}

impl fmt::Display for Grid<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut output = String::new();
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                output.push_str(format!("{} ", self.cells[NUM_COLS * row + col].value).as_str());
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}