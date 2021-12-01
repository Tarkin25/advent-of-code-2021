use day_01::*;

fn main() {
    let measurements = three_measurement_sums(read_input());
    let increments = count_increments(measurements);

    println!("{}", increments);
}