fn main() {
    use std::fs;

    let fish_ages = parse_numbers(fs::read_to_string("./input.txt").unwrap());
    let fish = fish_after_n_days(&fish_ages, 256);

    println!("{}", fish);
}

const RESET_AGE: usize = 6;
const NEW_AGE: usize = 8;

fn fish_after_n_days(fish_ages: &[usize], days: usize) -> usize {
    let by_age = group_by_age(&fish_ages);
    calculate_population(by_age, days)
}

fn group_by_age(fish_ages: &[usize]) -> [usize; NEW_AGE+1] {
    let mut by_age = [0_usize; NEW_AGE+1];

    for age in fish_ages {
        by_age[*age] += 1;
    }

    by_age
}

fn calculate_population(mut by_age: [usize; NEW_AGE+1], days: usize) -> usize {
    for _ in 0..days {
        pass_one_day(&mut by_age);
    }

    by_age.into_iter().sum()
}

fn pass_one_day(by_age: &mut [usize; NEW_AGE+1]) {
    let amount_zero = by_age[0]; // store how many fish are aged 0 --> that's how many new fish will be born

    for i in 1..by_age.len() {
        if i == RESET_AGE + 1 { // i = 7
            by_age[i-1] = by_age[i] + amount_zero; // by_age[6] = by_age[7] + amount_zero --> put all fish aged 7 and those whose age was reset from 0 to age 6
        } else {
            by_age[i-1] = by_age[i]; // put fish with the current age into age one day younger
        }
    }

    by_age[by_age.len()-1] = amount_zero; // put amount of new fish born by those aged 0 into age 8
}

fn parse_numbers(line: String) -> Vec<usize> {
    line.split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_by_age_works() {
        let fish_ages: Vec<usize> = vec![1,2,2,3,3,3,4,4,4,4];
        let by_age = group_by_age(&fish_ages);
        assert_eq!([0,1,2,3,4,0,0,0,0], by_age);
    }

    #[test]
    fn fish_after_18_days() {
        let fish_ages: Vec<usize> = vec![3,4,3,1,2];

        assert_eq!(fish_after_n_days(&fish_ages, 18), 26);
    }

    #[test]
    fn fish_after_80_days() {
        let fish_ages: Vec<usize> = vec![3,4,3,1,2];

        assert_eq!(fish_after_n_days(&fish_ages, 80), 5934);
    }

    #[test]
    fn fish_after_256_days() {
        let fish_ages: Vec<usize> = vec![3,4,3,1,2];

        assert_eq!(fish_after_n_days(&fish_ages, 256), 26984457539);
    }
}
