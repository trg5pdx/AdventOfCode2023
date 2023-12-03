use std::fs::File;
use std::io::{BufRead, BufReader};
use day1::calibration_sum;

fn main() {
    let path = "input.txt";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let mut lines = Vec::new();

    for l in buffered.lines() {
        lines.push(l.unwrap());
    }

    let result = calibration_sum(lines);

    println!("Result: {}", result);
}
