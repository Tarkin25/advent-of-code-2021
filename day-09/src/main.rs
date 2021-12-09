use day_09::{parse_map, part_1, part_2};

fn main() {
    let sum = part_1::<100, 100>(parse_map(input::lines!()));
    println!("Part 1: {}", sum);

    let product = part_2::<100, 100>(parse_map(input::lines!()));
    println!("Part 2: {}", product);
}
