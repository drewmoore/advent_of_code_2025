use std::fs::{read_to_string};

use advent_of_code_2025::{day_1};

#[test]
fn it_works() {
    let contents = read_to_string("tests/data/day_1.txt").expect("Should have been able to read the file");
    let result = day_1(contents);
    assert_eq!(999999, 0);
}
