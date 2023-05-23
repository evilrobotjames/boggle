
#[cfg(test)]
mod tests {
    use crate::dictionary::{list, tree::generate_root, testlist};
    use IsValidWord::{Yes, No, Never};
    
    fn test_round_trip(wordlist: &str) {
        assert_eq!(wordlist, generate_root(wordlist).to_list());
    }

    #[test]    
    fn test_round_trip_real() {
        test_round_trip(list::WORDLIST);
    }

    #[test]
    fn test_round_trip_artificial() {
        test_round_trip(testlist::WORDLIST_TEST_ARTIFICIAL);
    }

    #[test]
    fn test_round_trip_segment() {
        test_round_trip(testlist::WORDLIST_TEST_SEGMENT);
    }

    #[test]
    fn test_contains_word() {
        let root = generate_root(testlist::WORDLIST_TEST_SEGMENT);

        assert!(root.contains_word("cat") == Never);
        assert!(root.contains_word("icepick") == Yes);
        assert!(root.contains_word("icepi") == No);
        assert!(root.contains_word("icepicks") == Yes);
        assert!(root.contains_word("aicepicks") == Never);
        assert!(root.contains_word("icepicka") == Never);
        assert!(root.contains_word("") == No);
        assert!(root.contains_word("abcdefghijklmnopqrstuvwxyz") == Never);
    }

}