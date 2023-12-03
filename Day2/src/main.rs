use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use Day2::sum_id_possible_games;

fn main() {
    let path = "input.txt";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let mut lines = Vec::new();

    for l in buffered.lines() {
        lines.push(l.unwrap());
    }

    let result = sum_id_possible_games(lines);

    println!("Result: {}", result);
}
