fn main() {
    use std::fs;

    let fish_list = parse_numbers(fs::read_to_string("./input.txt").unwrap());
    let fish = fish_after_n_days(fish_list.into_iter(), 256);

    println!("{}", fish);
}

fn fish_after_n_days(fish_timers: impl Iterator<Item = usize>, days: usize) -> usize {
    let mut fish = 0;

    for timer in fish_timers {
        fish += fish_after_n_days_single(timer, days);
    }

    fish
}

fn fish_after_n_days_single(fish_timer: usize, days: usize) -> usize {
    if days == 0 {
        return 1;
    }

    let days = days -1;

    if fish_timer == 0 {
        fish_after_n_days_single(6, days) + fish_after_n_days_single(8, days)
    } else {
        fish_after_n_days_single(fish_timer-1, days)
    }
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
    fn fish_after_18_days() {
        let fish_list: Vec<usize> = vec![3,4,3,1,2];

        assert_eq!(fish_after_n_days(fish_list.into_iter(), 18), 26);
    }

    #[test]
    fn fish_after_80_days() {
        let fish_list: Vec<usize> = vec![3,4,3,1,2];

        assert_eq!(fish_after_n_days(fish_list.into_iter(), 80), 5934);
    }

    //#[test]
    fn fish_after_256_days() {
        let fish_list: Vec<usize> = vec![3,4,3,1,2];

        assert_eq!(fish_after_n_days(fish_list.into_iter(), 256), 26984457539);
    }
}
