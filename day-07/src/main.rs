use std::fs;
use day_07::{optimal_position, Part1, Part2Triangular};

fn main() {
    let numbers: Vec<usize> = fs::read_to_string("./input.txt").unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let (position, fuel) = optimal_position::<Part1>(numbers.clone()).unwrap();
    println!("[Part 1] position: {}, fuel: {}", position, fuel);

    let (position, fuel) = optimal_position::<Part2Triangular>(numbers).unwrap();
    println!("[Part 2] position: {}, fuel: {}", position, fuel);
}
