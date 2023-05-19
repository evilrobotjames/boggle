#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use crate::{game::{grid::{Direction, Grid}, self}, dictionary};

    fn go_all_directions(start: usize, expected_results: [Option<usize>; 4]) {
        let directions = [Direction::North, Direction::East, Direction::South, Direction::West];

        for (i, expected) in expected_results.iter().enumerate() {
            let result = Grid::go(start, directions[i]);

            let description = format!("start: {}, direction: {:?}, expected: {:?}, result: {:?}",
            start, directions[i], *expected, result);

            assert!(result == *expected, "{}", description);
        }
    }

    #[test]    
    fn test_go() {
        // Logical layout of cells:
        //
        // 0  1  2  3
        // 4  5  6  7
        // 8  9  10 11
        // 12 13 14 15

        // check all corners
        go_all_directions(0, [None, Some(1), Some(4), None]);
        go_all_directions(4, [Some(0), Some(5), Some(8), None]);
        go_all_directions(12, [Some(8), Some(13), None, None]);
        go_all_directions(15, [Some(11), None, None, Some(14)]);

        // and something in the center
        go_all_directions(5, [Some(1), Some(6), Some(9), Some(4)]);
    }

    #[test]
    fn test_new_random() {
        // Validate random grid can, at least, be formed.
        Grid::new_random();
    }

    #[test]
    fn test_new_from_values() {

        // Form grid from some values
        let values = vec!(
            "qu", "e", "a", "a",
            "a", "s", "a", "a",
            "a", "t", "n", "a",
            "a", "i", "o", "a",
        );
        let mut grid = Grid::new_from_values(values).expect("Grid should at least instantiate");

        // Definitely contains "question"
        game::solve(&mut grid, dictionary::contains_word);
        assert!(grid.words_found.contains(&String::from_str("question").unwrap()));
    }
}