use std::fs;
use day_07::optimal_position;

fn main() {
    let numbers: Vec<usize> = fs::read_to_string("./input.txt").unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let (position, fuel) = optimal_position(numbers).unwrap();


    println!("position: {}, fuel: {}", position, fuel);
}
