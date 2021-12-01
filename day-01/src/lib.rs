use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input() -> impl Iterator<Item = u32> {
    File::open("./input.txt")
        .map(|file| BufReader::new(file))
        .unwrap()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<u32>().unwrap())
}

pub fn count_increments(mut measurements: impl Iterator<Item = u32>) -> u32 {
    let mut increments = 0_u32;
    let mut current = if let Some(first) = measurements.next() {
        first
    } else {
        return 0
    };

    while let Some(next) = measurements.next() {
        if next > current {
            increments += 1;
        }

        current = next;
    }

    increments
}

pub struct ThreeMeasurementSums<I: Iterator<Item = u32>> {
    iter: I,
    first: u32,
    second: u32,
    third: u32,
}

impl<I: Iterator<Item = u32>> ThreeMeasurementSums<I> {
    pub fn new(mut iter: I) -> Self {
        let first = 0_u32;
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();

        Self {
            iter,
            first,
            second,
            third,
        }
    }
}

impl<I: Iterator<Item = u32>> Iterator for ThreeMeasurementSums<I> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.first = self.second;
        self.second = self.third;
        self.third = self.iter.next()?;

        Some(self.first + self.second + self.third)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_increments_works() {
        let measurements: Vec<u32> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(count_increments(measurements.into_iter()), 7);
    }

    #[test]
    fn three_measurement_sums_works() {
        let measurements = vec![1_u32, 2, 3, 4, 5].into_iter();
        let mut iter = ThreeMeasurementSums::new(measurements);

        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next(), Some(12));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn puzzle_2_works() {
        let measurements = vec![199_u32, 200, 208, 210, 200, 207, 240, 269, 260, 263].into_iter();
        let measurements = ThreeMeasurementSums::new(measurements);
        let increments = count_increments(measurements);

        assert_eq!(increments, 5);
    }
}
