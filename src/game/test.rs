#[cfg(test)]
mod tests {
    
    use crate::game::grid::{Direction, Grid};
    
// 0  1  2  3
// 4  5  6  7
// 8  9  10 11
// 12 13 14 15

    fn direction_and_go(start: usize, direction: Direction, expected_destination: Option<usize>) {

        let description = format!("start: {}, direction: {:?}, expected: {:?}",
                                  start, direction, expected_destination);

        let result = Grid::go(start, direction);

        let diagnostic = format!("{}, result: {:?}", description, result);

        if expected_destination.is_none() {
            assert!(result.is_none(), "{}", diagnostic);
        } else {
            assert!(result == expected_destination, "{}", diagnostic);
        }
    }

    #[test]    
    fn test_direction_and_go() {
        direction_and_go(0, Direction::North, None);
        direction_and_go(0, Direction::East, Some(1));
        direction_and_go(0, Direction::South, Some(4));
        direction_and_go(0, Direction::West, None);
        
        direction_and_go(1, Direction::North, None);
        direction_and_go(1, Direction::East, Some(2));
        direction_and_go(1, Direction::South, Some(5));
        direction_and_go(1, Direction::West, Some(0));

        direction_and_go(4, Direction::North, Some(0));
        direction_and_go(4, Direction::East, Some(5));
        direction_and_go(4, Direction::South, Some(8));
        direction_and_go(4, Direction::West, None);

        direction_and_go(12, Direction::North, Some(8));
        direction_and_go(12, Direction::East, Some(13));
        direction_and_go(12, Direction::South, None);
        direction_and_go(12, Direction::West, None);
        
        direction_and_go(15, Direction::North, Some(11));
        direction_and_go(15, Direction::East, None);
        direction_and_go(15, Direction::South, None);
        direction_and_go(15, Direction::West, Some(14));
    }
}