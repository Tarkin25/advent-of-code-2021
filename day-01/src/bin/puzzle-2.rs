use day_01::*;

fn main() {
    let measurements = ThreeMeasurementSums::new(read_input());
    let increments = count_increments(measurements);

    println!("{}", increments);
}