use std::fs::File;
use std::io::{BufRead, BufReader};

type Number = u32;
const BITS: usize = 32;

struct PowerConsumption {
    gamma_rate: Number,
    epsilon_rate: Number,
}

impl FromIterator<Number> for PowerConsumption {
    fn from_iter<T: IntoIterator<Item=Number>>(numbers: T) -> Self {
        let mut counts = vec![0; BITS];

        for number in numbers {
            for position in 0..BITS {
                let bit = number >> position & 1;

                if bit == 0 {
                    counts[position] -= 1;
                } else {
                    counts[position] += 1;
                }
            }
        }

        let mut gamma_rate = 0;

        for position in 0..BITS {
            gamma_rate |= ((counts[position] > 0) as Number) << position;
        }

        let mut mask = 0;

        for position in 0..significant_bits(gamma_rate) {
            mask |= 1 << position;
        }

        PowerConsumption {
            gamma_rate,
            epsilon_rate: gamma_rate ^ mask,
        }
    }
}

fn significant_bits(number: Number) -> usize {
    let mut bits = 0_usize;

    for position in 0..BITS {
        if number >> position & 1 == 1 {
            bits = position;
        }
    }

    bits+1
}

fn main() {
    let numbers = File::open("./input.txt")
        .map(BufReader::new)
        .unwrap()
        .lines()
        .map(Result::unwrap)
        .map(|line| Number::from_str_radix(line.as_str(), 2).unwrap());

    let PowerConsumption { gamma_rate, epsilon_rate } = numbers.collect();

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

        let PowerConsumption { gamma_rate, epsilon_rate } = FromIterator::from_iter(numbers);

        assert_eq!(gamma_rate, 0b_10110);
        assert_eq!(epsilon_rate, 0b_01001);
    }

    #[test]
    fn masked_negation() {
        let x = 0b_00001010;
        let mask = 0b_00001111;
        let negated = x ^ mask;

        assert_eq!(0b_00000101, negated);
    }
}
