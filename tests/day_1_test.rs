use advent_of_code_2025::{day_1};

// TODO: meet requirements: the number of times the dial is left pointing at 0 after any rotation in the sequence
//       NOT number of traversals... read the instructions !
// TODO: R + L are flipped, turning right decreases numbers...
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

// Answer is 1011
