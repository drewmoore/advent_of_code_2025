use std::fs::read_to_string;

use advent_of_code_2025::day_2;

#[test]
fn it_works() {
    // 82 -> 52 -> 0 -> 95 -> 55 -> 0 -> 99 -> 0 -> 14 -> 32
    let contents = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    let result = day_2(contents);
    assert_eq!(result, 6);
}

#[test]
fn it_works_for_large_rotations() {
    let contents = "R1000";
    let result = day_2(contents);
    assert_eq!(result, 10);
}

#[test]
fn it_solves_the_puzzle() {
    let contents =
        read_to_string("tests/data/day_2.txt").expect("Should have been able to read the file");
    let result = day_2(&contents);

    // TODO: got 5593, which is too low
    assert_eq!(result, 123);
}