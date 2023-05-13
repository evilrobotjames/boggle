use rand::Rng;

pub const NUM_DICE: usize = 16;
pub const NUM_SIDES: usize = 6;

pub static DICE: [[&str; NUM_SIDES]; NUM_DICE] = [
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

pub fn roll(sides: [&str; NUM_SIDES]) -> &str {
    let mut rng = rand::thread_rng();
    let side_index = rng.gen_range(0..NUM_SIDES);

    sides[side_index]
}