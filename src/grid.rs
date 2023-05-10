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

pub struct Grid <'a> {
    cells: [Cell<'a>; NUM_CELLS],
}

impl <'a> Grid <'_> {

    //pub fn new() -> Grid <'a> {
    pub fn new() {
        // Create a new grid, with some random stuff in cells.

        let mut dice_order: Vec<usize> = (0..NUM_CELLS).collect();
        dice_order.shuffle(&mut rand::thread_rng());

        println!("{:?}", dice_order);

        for dice_index in dice_order {
            let mut rng = rand::thread_rng();
            let side_index = rng.gen_range(0..NUM_SIDES);


        }


        //Grid {
            //cells: Cell {traversed_already: false, value: DICE[3][4]}
        //}
    }

}

impl fmt::Display for Grid<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Result::Ok(())
        /*
        let output = String::new();
        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                match write!(&mut output, "{} ", self.cells[NUM_COLS * row + col].value) {
                    Ok => pass,
                    Err(s) => return Err(s),
                }
            }
            write!(f, "\n");
        }

        Ok(())
        */
    }
}