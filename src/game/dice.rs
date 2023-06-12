use rand::Rng;

pub const NUM_SIDES: usize = 6;

pub type DiceSet = Vec<[&'static str; NUM_SIDES]>;

lazy_static! {
    pub static ref DICE_REGULAR: DiceSet = vec![
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
}

lazy_static! {
    pub static ref DICE_BIG: DiceSet = vec![
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
}

lazy_static! {
    pub static ref DICE_SUPER: DiceSet = vec![
        ["a", "a", "f", "r", "r", "s"],
        ["a", "e", "e", "e", "e", "e"],
        ["a", "e", "e", "o", "o", "o"],
        ["a", "f", "i", "r", "r", "s"],
        ["a", "d", "e", "i", "i", "o"],
        ["a", "e", "n", "n", "n", "n"],
        ["a", "e", "e", "e", "e", "m"],
        ["a", "e", "g", "m", "m", "u"],
        ["a", "g", "m", "n", "n", "n"],
        ["a", "i", "l", "m", "m", "n"],
        ["a", "i", "n", "o", "o", "u"],
        ["a", "i", "r", "s", "s", "y"],
        ["qu", "th", "er", "he", "he", "an"],
        ["b", "j", "k", "x", "x", "z"],
        ["c", "e", "n", "s", "s", "t"],
        ["c", "d", "l", "n", "n", "n"],
        ["c", "i", "i", "t", "t", "t"],
        ["c", "i", "p", "s", "s", "t"],
        ["c", "g", "n", "u", "u", "y"],
        ["d", "h", "n", "o", "o", "t"],
        ["d", "h", "l", "o", "o", "r"],
        ["d", "h", "n", "o", "o", "w"],
        ["d", "l", "n", "o", "o", "r"],
        ["e", "i", "l", "r", "r", "s"],
        ["e", "i", "l", "s", "s", "t"],
        ["e", "l", "p", "s", "s", "t"],
        ["e", "o", ".", ".", ".", "."], //
        ["e", "t", "t", "t", "t", "o"],
        ["e", "s", "s", "s", "s", "u"],
        ["g", "r", "r", "v", "v", "w"],
        ["h", "r", "s", "t", "t", "v"],
        ["h", "p", "r", "s", "s", "t"],
        ["i", "r", "s", "y", "y", "y"],
        ["j", "qu", "w", "x", "x", "z"],
        ["n", "o", "t", "u", "u", "w"],
        ["o", "o", "t", "t", "t", "u"],
    ];
}

lazy_static! {
    pub static ref DICE_SETS: [DiceSet; 3] = [
        DICE_REGULAR.to_vec(),
        DICE_BIG.to_vec(),
        DICE_SUPER.to_vec(),
    ];
}


pub fn roll(sides: [&str; NUM_SIDES]) -> &str {
    let mut rng = rand::thread_rng();
    let side_index = rng.gen_range(0..NUM_SIDES);

    sides[side_index]
}