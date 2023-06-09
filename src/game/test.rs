#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use crate::{game::{grid::{Grid, DIRECTIONS, invalid_cell_count_diagnostic, side_length_from_cell_count, lookup_dice_set_by_cell_count, lookup_dice_set_by_side_length}, self}, dictionary};

    #[test]
    fn test_side_length_from_cell_count() {
        let f = side_length_from_cell_count;
        assert!(f(1) == Some(1));
        assert!(f(3) == None);
        assert!(f(4) == Some(2));
        assert!(f(5) == None);
        assert!(f(16) == Some(4));
        assert!(f(24) == None);
        assert!(f(25) == Some(5));
        assert!(f(36) == Some(6));
    }

    #[test]
    fn test_invalid_cell_count_diagnostic() {
        let diagnostic = invalid_cell_count_diagnostic(4);
        assert!(diagnostic == "Cells specified: 4.  Require: 16, 25, 36");
    }

    #[test]
    fn test_lookup_dice_set_by_cell_count() {
        let f = lookup_dice_set_by_cell_count;
        assert!(f(3).is_err()); 
        assert!(f(4).is_err()); 
        assert!(f(15).is_err()); 
        assert!(f(16).unwrap().len() == 16); 
        assert!(f(25).unwrap().len() == 25); 
        assert!(f(36).unwrap().len() == 36); 
    }

    #[test]
    fn test_lookup_dice_set_by_side_length() {
        let f = lookup_dice_set_by_side_length;
        assert!(f(3).is_err());
        assert!(f(4).unwrap().len() == 16); 
        assert!(f(5).unwrap().len() == 25); 
        assert!(f(6).unwrap().len() == 36);
        assert!(f(7).is_err());       
        assert!(f(9).is_err()); 
    }

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
        // Logical layout of cells:
        // 0  1  2  3
        // 4  5  6  7
        // 8  9  10 11
        // 12 13 14 15
        let grid = Grid::new_random(4).unwrap();

        for cell in [0, 1, 2, 3] {
            assert!(grid.is_north_edge(cell));
        }
        for cell in [4, 5, 6] {
            assert!(!grid.is_north_edge(cell));
        }
        for cell in [3, 7, 15] {
            assert!(grid.is_east_edge(cell));
        }
        for cell in [0, 5, 14] {
            assert!(!grid.is_east_edge(cell));
        }
        for cell in [12, 13, 15] {
            assert!(grid.is_south_edge(cell));
        }
        for cell in [0, 5, 11] {
            assert!(!grid.is_south_edge(cell));
        }
        for cell in [0, 4, 12] {
            assert!(grid.is_west_edge(cell));
        }
        for cell in [5, 10, 15] {
            assert!(!grid.is_west_edge(cell));
        }
    }

    #[test]    
    fn test_go() {
        // Logical layout of cells:
        // 0  1  2  3
        // 4  5  6  7
        // 8  9  10 11
        // 12 13 14 15
        let grid = game::new_random(4).unwrap();

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
        game::new_random(4).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_new_random_fails() {
        // Validate random grid fails when given non-square numbers.
        assert!(Grid::new_random(15).expect_err("").as_str() == 
                "Cells specified: 15.  Require: 16, 25, 36");
        assert!(Grid::new_random(24).expect_err("").as_str() ==
                "Cells specified: 24.  Require: 16, 25, 36");
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