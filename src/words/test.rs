#[cfg(test)]
mod tests {
    use crate::words::{tree::generate_root, testlist::WORDLIST_TEST_ARTIFICIAL, testlist::WORDLIST_TEST_SEGMENT};
    
/*
    use crate::words::list;
    #[test]    
    fn test_round_trip_real() {
        assert!(list::WORDLIST == generate_root(list::WORDLIST).to_list());
    }
*/
    #[test]
    fn test_round_trip_artificial() {
        assert!(WORDLIST_TEST_ARTIFICIAL == generate_root(WORDLIST_TEST_ARTIFICIAL).to_list());
    }

    #[test]
    fn test_round_trip_segment() {
        assert!(WORDLIST_TEST_SEGMENT == generate_root(WORDLIST_TEST_SEGMENT).to_list());
    }

}