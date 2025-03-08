// A function that takes a string and returns its length
fn string_length(s: &str) -> usize {
    s.chars().count()
}

// Test the function with some examples
#[test]
fn test_string_length() {
    assert_eq!(3, string_length("abc"));
    assert_eq!(0, string_length(""));
    assert_eq!(1, string_length("a"));
}
