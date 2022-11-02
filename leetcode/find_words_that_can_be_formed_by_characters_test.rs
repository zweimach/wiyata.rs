#![cfg(test)]
use super::find_words_that_can_be_formed_by_characters::count_characters;

#[test]
fn it_returns_the_character_counts() {
    assert_eq!(
        count_characters(
            vec![
                "cat".to_string(),
                "bt".to_string(),
                "hat".to_string(),
                "tree".to_string()
            ],
            "atach".to_string()
        ),
        6
    );
    assert_eq!(
        count_characters(
            vec![
                "hello".to_string(),
                "world".to_string(),
                "leetcode".to_string()
            ],
            "welldonehoneyr".to_string()
        ),
        10
    );
}
