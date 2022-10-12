#![cfg(test)]
use super::power_of_thor::power_of_thor;

#[test]
fn it_returns_direction_and_new_coordinates() {
    assert_eq!(power_of_thor(&1, &4, &11, &6), ("NW".to_string(), 10, 5));
    assert_eq!(power_of_thor(&5, &7, &1, &3), ("SE".to_string(), 2, 4));
    assert_eq!(power_of_thor(&5, &15, &11, &15), ("W".to_string(), 10, 15));
    assert_eq!(power_of_thor(&2, &10, &2, &8), ("S".to_string(), 2, 9));
}
