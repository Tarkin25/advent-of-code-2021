use std::fs::File;
use std::io::{BufRead, BufReader};

type Number = u32;

struct PowerConsumption {
    gamma_rate: Number,
    epsilon_rate: Number,
}

impl PowerConsumption {
    fn new(numbers: Vec<Number>, significant_bits: usize) -> Self {
        let mut gamma_rate: Number = 0;
        let mut epsilon_rate: Number = 0;

        fn apply(common_bit: CommonBit, number: &mut Number, position: usize) {
            match common_bit {
                CommonBit::Zero => *number |= 0 << position,
                CommonBit::One => *number |= 1 << position,
                CommonBit::Equal => {}
            }
        }

        for position in 0..significant_bits {
            apply(CommonBit::most_at(numbers.iter(), position), &mut gamma_rate, position);
            apply(CommonBit::least_at(numbers.iter(), position), &mut epsilon_rate, position);
        }

        Self {
            gamma_rate,
            epsilon_rate,
        }
    }
}

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

fn main() {
    let numbers: Vec<Number> = File::open("./input.txt")
        .map(BufReader::new)
        .unwrap()
        .lines()
        .map(Result::unwrap)
        .map(|line| Number::from_str_radix(line.as_str(), 2).unwrap())
        .collect();

    let PowerConsumption { gamma_rate, epsilon_rate } = PowerConsumption::new(numbers, 12);

    println!("{}", gamma_rate * epsilon_rate);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
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

        let PowerConsumption { gamma_rate, epsilon_rate } = PowerConsumption::new(numbers, 5);

        assert_eq!(gamma_rate, 0b_10110);
        assert_eq!(epsilon_rate, 0b_01001);
    }
}
