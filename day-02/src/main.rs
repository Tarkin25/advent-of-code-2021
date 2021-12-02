use std::fs::File;
use std::io::{BufRead, BufReader};
use day_02::{parse_instructions, Position};

fn main() {
    let lines = File::open("./input.txt").map(BufReader::new).unwrap().lines().map(Result::unwrap);
    let position: Position = parse_instructions(lines).map(Result::unwrap).collect();

    println!("{}", position.horizontal_position * position.depth);
}