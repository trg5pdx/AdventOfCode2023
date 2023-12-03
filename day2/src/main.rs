use std::fs::File;
use std::io::{BufRead, BufReader};
use day2::*;

fn main() {
    let path = "input.txt";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let mut lines = Vec::new();

    for l in buffered.lines() {
        lines.push(l.unwrap());
    }

    println!("Result for part 1: {}", sum_id_possible_games(&lines));
    println!("Result for part 2: {}", sum_of_pow_min_cubes(&lines));
}
