const UNIQUE_DIGIT_COUNTS: [usize; 4] = [2,3,4,7];

fn main() {
    let count = part_1(input::lines!());
    println!("part 1: {}", count);
}

fn part_1(lines: impl IntoIterator<Item = String>) -> usize {
    lines
        .into_iter()
        .map(|line| {
            let mut split = line.split(" | ").skip(1);
            split.next().unwrap().to_string()
        })
        .map(|line| {
            line.split(" ")
                .map(|digits| digits.len())
                .collect::<Vec<_>>()
        })
        .flatten()
        .filter(|digit_count| UNIQUE_DIGIT_COUNTS.contains(digit_count))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let lines = [
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ].map(ToString::to_string);

        assert_eq!(part_1(lines), 26);
    }
}
