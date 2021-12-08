mod util;

use std::collections::HashMap;
use std::str::SplitWhitespace;
use crate::util::{digit_to_segments, DIGITS, len_to_digit};

const UNIQUE_DIGIT_COUNTS: [usize; 4] = [2,3,4,7];

fn main() {
    let count = part_1(input::lines!());
    println!("part 1: {}", count);

    let sum = part_2(input::lines!());
    println!("part 2: {}", sum);
}

fn part_2(lines: impl IntoIterator<Item = String>) -> usize {
    lines
        .into_iter()
        .map(|line: String| {
            let mut split = line.split(" | ");
            let mappings = determine_mappings(split.next().unwrap()).unwrap();
            let output_value = read_output_patterns(split.next().unwrap(), mappings);
            output_value
        })
        .map(vec_to_num)
        .sum()
}

fn vec_to_num(digits: Vec<usize>) -> usize {
    let mut num_str = String::with_capacity(4);

    for digit in digits {
        num_str.push_str(&digit.to_string());
    }

    num_str.parse().unwrap()
}

fn determine_mappings(patterns: &str) -> Option<HashMap<char, char>> {
    let mut possible_mappings: HashMap<char, Vec<char>> = HashMap::new();

    for c in 'a'..='g' {
        possible_mappings.insert(c, ('a'..='g').collect());
    }

    apply_uniquely_sized_digits(&mut possible_mappings, patterns.split_whitespace());

    determine_mappings_rec(possible_mappings, patterns)
}

fn determine_mappings_rec(mappings: HashMap<char, Vec<char>>, patterns: &str) -> Option<HashMap<char, char>> {
    // exit condition
    if mappings.values().all(|m| m.len() == 1) {
        let mappings = mappings.iter().map(|(key, value)| (*key, *&value[0])).collect::<HashMap<_, _>>();

        return if patterns.split_whitespace().all(|pattern| can_build_digit(pattern, &mappings)) {
            Some(mappings)
        } else {
            None
        }
    }

    // try a possible mapping

    // get first mapping with more than 1 possibility
    let (key, values) = mappings.iter().find(|(_, values)| values.len() > 1).unwrap();

    for value in values {
        let mut mappings = mappings.clone();

        mappings.get_mut(key).unwrap().clear();
        mappings.get_mut(key).unwrap().push(*value);

        for (_, values) in mappings.iter_mut().filter(|(k, _)| *k != key) {
            values.retain(|v| v != value);
        }

        if let Some(mappings) = determine_mappings_rec(mappings, patterns) {
            return Some(mappings);
        }
    }

    None
}

fn can_build_digit(pattern: &str, mappings: &HashMap<char, char>) -> bool {
    DIGITS
        .iter()
        .filter(|digit| digit.0.len() == pattern.len())
        .any(|digit| digit_matches_pattern(digit.0, pattern, mappings))
}

fn apply_uniquely_sized_digits(possible_mappings: &mut HashMap<char, Vec<char>>, patterns: SplitWhitespace) {
    for pattern in patterns {
        if let Some(digit) = len_to_digit(pattern.len()) {
            let correct_segments = digit_to_segments(digit);

            for (letter, mappings) in possible_mappings.iter_mut() {
                if pattern.contains(*letter) {
                    mappings.retain(|mapping| correct_segments.0.contains(mapping));
                } else {
                    mappings.retain(|mapping| !correct_segments.0.contains(mapping));
                }
            }
        }
    }
}

fn read_output_patterns(patterns: &str, mappings: HashMap<char, char>) -> Vec<usize> {
    let mut result = vec![];

    for pattern in patterns.split_whitespace() {
        let (_, digit) = DIGITS.iter().find(|digit| digit_matches_pattern(digit.0, pattern, &mappings)).unwrap();
        result.push(*digit);
    }

    result
}

fn digit_matches_pattern(digit: &[char], pattern: &str, mappings: &HashMap<char, char>) -> bool {
    digit.len() == pattern.len() && pattern.chars().all(|c| digit.contains(&mappings[&c]))
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

    #[test]
    fn part_2_works() {
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

        assert_eq!(part_2(lines), 61229);
    }

    #[test]
    fn analyze_patterns_works() {
        let patterns = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab";
        let patterns = determine_mappings(patterns).unwrap();

        assert_eq!(patterns[&'a'], 'c');
        assert_eq!(patterns[&'b'], 'f');
        assert_eq!(patterns[&'c'], 'g');
        assert_eq!(patterns[&'d'], 'a');
        assert_eq!(patterns[&'e'], 'b');
        assert_eq!(patterns[&'f'], 'd');
        assert_eq!(patterns[&'g'], 'e');
    }

    #[test]
    fn read_output_patterns_works() {
        let mut mappings = HashMap::new();

        for (key, value) in [
            ('a', 'c'),
            ('b', 'f'),
            ('c', 'g'),
            ('d', 'a'),
            ('e', 'b'),
            ('f', 'd'),
            ('g', 'e'),
        ] {
            mappings.insert(key, value);
        }

        let patterns = "cdfeb fcadb cdfeb cdbaf";
        assert_eq!(read_output_patterns(patterns, mappings), vec![5,3,5,3]);
    }

    #[test]
    fn can_build_digit_works() {
        let mut mappings = HashMap::new();
        mappings.insert('d', 'a');
        mappings.insert('a', 'c');
        mappings.insert('b', 'f');

        assert!(can_build_digit("dab", &mappings));

        mappings.insert('a', 'f');
        mappings.insert('b', 'c');

        assert!(can_build_digit("dab", &mappings));

        mappings.insert('a', 'd');

        assert!(!can_build_digit("dab", &mappings));
    }
}
