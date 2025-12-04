use std::fs::{read_to_string};

use advent_of_code_2025::{day_1};

#[test]
fn it_works() {
    // 50 -> 9 -> 19 -> 37 -> 15 -> 98 -> 0 -> 0 -> 0
    let contents = "R41
L10
L18
R22
R17
L2
L200
R300";
    let result = day_1(contents);
    assert_eq!(result, 3);
}

#[test]
fn it_solves_the_puzzle() {
    let contents = read_to_string("tests/data/day_1.txt").expect("Should have been able to read the file");
    let result = day_1(&contents);

    assert_eq!(result, 1011);
}
