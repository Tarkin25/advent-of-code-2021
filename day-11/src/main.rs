fn main() {
    let flashes = day_11::part_1(input::lines!(), 100);
    println!("Part 1: {}", flashes);

    let step = day_11::part_2(input::lines!());
    println!("Part 2: {}", step);
}
