use std::fs::File;
use std::io::{BufRead, BufReader};

type Number = u32;

#[derive(Copy, Clone)]
enum CommonBit {
    Zero,
    One,
    Equal,
}

impl CommonBit {
    pub fn most_at<'a>(numbers: impl Iterator<Item = &'a Number>, position: usize) -> Self {
        let mut count = 0;

        for number in numbers {
            let bit = number >> position & 1;

            if bit == 0 {
                count -= 1;
            } else {
                count += 1;
            }
        }

        if count < 0 {
            CommonBit::Zero
        } else if count > 0 {
            CommonBit::One
        } else {
            CommonBit::Equal
        }
    }

    pub fn least_at<'a>(numbers: impl Iterator<Item = &'a Number>, position: usize) -> Self {
        Self::most_at(numbers, position).invert()
    }

    fn invert(self) -> Self {
        match self {
            CommonBit::One => CommonBit::Zero,
            CommonBit::Zero => CommonBit::One,
            CommonBit::Equal => CommonBit::Equal,
        }
    }
}

fn oxygen_generator_rating(numbers: Vec<Number>, significant_bits: usize) -> Number {
    determine_rating(numbers, significant_bits, |numbers, position| {
        if let CommonBit::Zero = CommonBit::most_at(numbers.iter(), position) {
            0
        } else {
            1
        }
    })
}

fn co2_scrubber_rating(numbers: Vec<Number>, significant_bits: usize) -> Number {
    determine_rating(numbers, significant_bits, |numbers, position| {
        if let CommonBit::One = CommonBit::least_at(numbers.iter(), position) {
            1
        } else {
            0
        }
    })
}

fn determine_rating<F>(mut numbers: Vec<Number>, significant_bits: usize, bit_criteria_provider: F) -> Number
where
    F: Fn(&[Number], usize) -> Number
{
    let mut position = significant_bits;

    while numbers.len() > 1 {
        {
            let position = position -1;
            let bit_criteria = bit_criteria_provider(&numbers, position);

            numbers = numbers.into_iter()
                .filter(|number| number >> position & 1 == bit_criteria)
                .collect();
        }

        position -= 1;
    }

    numbers[0]
}

fn main() {
    let numbers: Vec<Number> = File::open("./input.txt")
        .map(BufReader::new)
        .unwrap()
        .lines()
        .map(Result::unwrap)
        .map(|line| Number::from_str_radix(line.as_str(), 2).unwrap())
        .collect();

    let oxygen_generator_rating = oxygen_generator_rating(numbers.clone(), 12);
    let co2_scrubber_rating = co2_scrubber_rating(numbers, 12);

    println!("{}", oxygen_generator_rating * co2_scrubber_rating);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn oxygen_generator_rating_works() {
        let numbers: Vec<Number> = vec![
            0b_00100,
            0b_11110,
            0b_10110,
            0b_10111,
            0b_10101,
            0b_01111,
            0b_00111,
            0b_11100,
            0b_10000,
            0b_11001,
            0b_00010,
            0b_01010,
        ];

        assert_eq!(oxygen_generator_rating(numbers, 5), 23);
    }

    #[test]
    fn co2_scrubber_rating_works() {
        let numbers: Vec<Number> = vec![
            0b_00100,
            0b_11110,
            0b_10110,
            0b_10111,
            0b_10101,
            0b_01111,
            0b_00111,
            0b_11100,
            0b_10000,
            0b_11001,
            0b_00010,
            0b_01010,
        ];

        assert_eq!(co2_scrubber_rating(numbers, 5), 10);
    }
}
