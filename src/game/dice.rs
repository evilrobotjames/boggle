use rand::Rng;

pub const NUM_SIDES: usize = 6;

pub const DICE_REGULAR_NUM: usize = 16;
pub static DICE_REGULAR: [[&str; NUM_SIDES]; DICE_REGULAR_NUM] = [
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

pub const DICE_BIG_NUM: usize = 25;
pub static DICE_BIG: [[&str; NUM_SIDES]; DICE_BIG_NUM] = [
    ["a", "a", "a", "f", "r", "s"],
    ["a", "a", "e", "e", "e", "e"],
    ["a", "a", "f", "i", "r", "s"],
    ["a", "d", "e", "n", "n", "n"],
    ["a", "e", "e", "e", "e", "m"],

    ["a", "e", "e", "g", "m", "u"],
    ["a", "e", "g", "m", "n", "n"],
    ["a", "f", "i", "r", "s", "y"],
    ["b", "j", "k", "qu", "x", "z"],
    ["c", "c", "e", "n", "s", "t"],

    ["c", "e", "i", "i", "l", "t"],
    ["c", "e", "i", "l", "p", "t"],
    ["c", "e", "i", "p", "s", "t"],
    ["d", "d", "h", "n", "o", "t"],
    ["d", "h", "h", "l", "o", "r"],

    ["d", "h", "l", "n", "o", "r"],
    ["d", "h", "l", "n", "o", "r"],
    ["e", "i", "i", "i", "t", "t"],
    ["e", "m", "o", "t", "t", "t"],
    ["e", "n", "s", "s", "s", "u"],

    ["f", "i", "p", "r", "s", "y"],
    ["g", "o", "r", "r", "v", "w"],
    ["i", "p", "r", "r", "r", "y"],
    ["n", "o", "o", "t", "u", "w"],
    ["o", "o", "o", "t", "t", "u"],
];

pub fn roll(sides: [&str; NUM_SIDES]) -> &str {
    let mut rng = rand::thread_rng();
    let side_index = rng.gen_range(0..NUM_SIDES);

    sides[side_index]
}