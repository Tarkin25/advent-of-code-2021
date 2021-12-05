use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use crate::line::{DirectionType, Line};
use crate::point::Point;

pub mod point;
pub mod line;

fn overlap_diagram(lines: impl Iterator<Item = Line>) -> HashMap<Point, u32> {
    let mut overlaps = HashMap::new();

    for line in lines {
        for point in line.traverse() {
            let overlap_count = overlaps.entry(point).or_insert(0);
            *overlap_count += 1;
        }
    }

    overlaps
}

fn parse_lines(lines: impl Iterator<Item = String>) -> impl Iterator<Item = Line> {
    lines
        .map(|line| Line::from_str(line.as_str()).unwrap())
}

fn count_dangerous_overlaps(lines: impl Iterator<Item = Line>) -> usize {
    let lines = lines.filter(|line| {
        match line.direction() {
            DirectionType::Horizontal | DirectionType::Vertical => true,
            _ => false,
        }
    });
    let overlap_diagram = overlap_diagram(lines);

    overlap_diagram.values()
        .filter(|overlaps| **overlaps > 1)
        .count()
}

fn main() {
    let lines = File::open("./input.txt")
        .map(BufReader::new)
        .unwrap()
        .lines()
        .map(|line| line.unwrap());

    let lines = parse_lines(lines);
    let overlap_count = count_dangerous_overlaps(lines);

    println!("{}", overlap_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_diagram_works() {
        let lines = [
            Line { start: (0, 0).into(), end: (1, 0).into()},
            Line { start: (0, 0).into(), end: (0, 1).into()},
        ];

        let overlap_diagram = overlap_diagram(lines.into_iter());

        assert_eq!(overlap_diagram.get(&(0, 0).into()), Some(&2));
        assert_eq!(overlap_diagram.get(&(1, 0).into()), Some(&1));
        assert_eq!(overlap_diagram.get(&(0, 1).into()), Some(&1));
        assert_eq!(overlap_diagram.get(&(1, 1).into()), None);
    }

    #[test]
    fn count_dangerous_overlaps_works() {
        let lines = [
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2"
        ]
            .map(ToString::to_string);
        let overlap_counter = count_dangerous_overlaps(parse_lines(lines.into_iter()));

        assert_eq!(overlap_counter, 5);
    }
}
