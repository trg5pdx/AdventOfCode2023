use std::fs::File;
use std::io::{BufRead, BufReader};
use day5::*;

fn main() {
    let path = "input.txt";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let mut lines = String::new();

    for l in buffered.lines() {
        lines += &l.unwrap();
    }

    println!("Result for part 1: {}", get_lowest_seed_location(&lines));
}
