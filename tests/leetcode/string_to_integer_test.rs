#[cfg(test)]
use wiyata::leetcode::string_to_integer::my_atoi;

#[test]
fn it_returns_an_integer_from_parsed_digit_characters() {
    assert_eq!(my_atoi("42".to_string()), 42);
    assert_eq!(my_atoi("   -42".to_string()), -42);
    assert_eq!(my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(my_atoi("words and 987".to_string()), 0);
    assert_eq!(my_atoi("-91283472332".to_string()), -2147483648);
    assert_eq!(my_atoi("- 100".to_string()), 0);
    assert_eq!(my_atoi("+-12".to_string()), 0);
}
