use std::env;
use std::fs::{read_to_string};

use advent_of_code_2025::{day_1};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_number = &args[1];

    println!("hello {day_number}");

    if day_number == "1" {
        let contents = read_to_string(&args[2]).expect("Should have been able to read the file");
        let result = day_1(&contents);
        println!("Secret number is {result}");


    }

}
