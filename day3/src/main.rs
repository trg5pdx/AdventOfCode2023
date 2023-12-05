use std::fs::File;
use std::io::{BufRead, BufReader};
use day3::*;

fn main() {
    let path = "input.txt";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let mut lines: Vec<Vec<char>> = Vec::new();

    for l in buffered.lines() {
        lines.push(l.unwrap().chars().collect());
    }

    println!("Result {}", sum_part_numbers(lines));
}
