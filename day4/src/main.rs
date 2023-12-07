use std::fs::File;
use std::io::{BufRead, BufReader};
use day4::*;

fn main() {
    let path = "input.txt";
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    let mut lines: Vec<String> = Vec::new();

    for l in buffered.lines() {
        lines.push(l.unwrap());
    }

    println!("Result for part 1: {}", sum_of_winning_cards(&lines));
    println!("Result for part 2: {}", number_of_cards_won(&lines));
}
