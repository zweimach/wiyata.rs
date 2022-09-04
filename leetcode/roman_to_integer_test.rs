#[cfg(test)]
use crate::leetcode::roman_to_integer::roman_to_int;

#[test]
fn it_returns_parsed_integer() {
    assert_eq!(roman_to_int("III".into()), 3);
    assert_eq!(roman_to_int("LVIII".into()), 58);
    assert_eq!(roman_to_int("MCMXCIV".into()), 1994);
}
