#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use crate::{game::{grid::{Grid, DIRECTIONS}, self}, dictionary};

    fn go_all_directions(grid: &Grid, start: usize, expected_results: [Option<usize>; 8]) {

        for (i, expected) in expected_results.iter().enumerate() {
            let result = grid.go(start, DIRECTIONS[i]);

            let description = format!("start: {}, direction: {:?}, expected: {:?}, result: {:?}",
            start, DIRECTIONS[i], *expected, result);

            assert!(result == *expected, "{}", description);
        }
    }

    #[test]    
    fn test_edges() {
        let grid = Grid::new_random(4);
        assert!(grid.is_north_edge(0));
        assert!(grid.is_north_edge(1));
        assert!(grid.is_north_edge(2));
        assert!(grid.is_north_edge(3));
        assert!(!grid.is_north_edge(4));
        assert!(!grid.is_north_edge(5));
        assert!(!grid.is_north_edge(6));
    }


    #[test]    
    fn test_go() {
        // Logical layout of cells:
        // 0  1  2  3
        // 4  5  6  7
        // 8  9  10 11
        // 12 13 14 15
        let grid = Grid::new_random(4);

        // check all corners
        go_all_directions(&grid, 0, [None, None, Some(1), Some(5), Some(4), None, None, None]);
        go_all_directions(&grid, 3, [None, None, None, None, Some(7), Some(6), Some(2), None]);
        go_all_directions(&grid, 12, [Some(8), Some(9), Some(13), None, None, None, None, None]);
        go_all_directions(&grid, 15, [Some(11), None, None, None, None, None, Some(14), Some(10)]);

        // and something in the center
        go_all_directions(&grid, 5, [Some(1), Some(2), Some(6), Some(10), Some(9), Some(8), Some(4), Some(0)]);
    }

    #[test]
    fn test_new_random() {
        // Validate random grid can, at least, be formed.
        Grid::new_random(4);
    }

    #[test]
    #[should_panic]
    fn test_new_random_fails() {
        // Validate random grid can, at least, be formed.
        Grid::new_random(5);
        
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
        println!("{:?}", grid.words_found);
        assert!(grid.words_found.contains(&String::from_str("question").unwrap()));
    }
}