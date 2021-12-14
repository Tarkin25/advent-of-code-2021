#![feature(string_remove_matches)]

use std::collections::HashSet;
use std::ops::Sub;
use std::str::FromStr;

pub fn part_1<I: IntoIterator<Item=String>>(lines: I) -> usize {
    let mut lines = lines.into_iter().filter(|line| !line.is_empty()).peekable();

    let mut points = HashSet::new();

    while let Some(line) = lines.next_if(|line| !line.starts_with("fold")) {
        points.insert(Point::from_str(&line).unwrap());
    }

    let fold_instruction = lines.next().map(FoldInstruction::try_from).unwrap().unwrap();

    points = points.into_iter()
        .map(|point| {
            let Point { x, y } = point;

            match fold_instruction.axis {
                Axis::Y => Point { x, y: fold_instruction.position - y.diff(fold_instruction.position)},
                Axis::X => Point { x: fold_instruction.position - x.diff(fold_instruction.position), y }
            }
        })
        .collect();

    points.len()
}

#[derive(Debug, Default, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");

        let x = parse(split.next())?;
        let y = parse(split.next())?;

        Ok(Self { x, y })
    }
}

#[derive(Debug, Copy, Clone)]
enum Axis {
    X,
    Y,
}

#[derive(Debug, Copy, Clone)]
struct FoldInstruction {
    axis: Axis,
    position: usize,
}

impl TryFrom<String> for FoldInstruction {
    type Error = String;

    fn try_from(mut s: String) -> Result<Self, Self::Error> {
        s.remove_matches("fold along ");

        let mut split = s.split("=");
        let axis = match split.next().ok_or("expected value".to_string())? {
            "x" => Axis::X,
            "y" => Axis::Y,
            axis => return Err(format!("unexpected axis '{}'", axis))
        };
        let position = parse(split.next())?;

        Ok(Self {axis, position})
    }
}

fn parse(s: Option<&str>) -> Result<usize, String> {
    s.ok_or("expected value".to_string())
        .and_then(|s| s.parse::<usize>().map_err(|e| e.to_string()))
}

trait Diff: Ord + Sub + Sized {
    fn diff(self, b: Self) -> Self::Output;
}

impl<T: Ord + Sub + Sized> Diff for T {
    fn diff(self, b: Self) -> Self::Output {
        if self >= b {
            self - b
        } else {
            b - self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let lines = [
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
        ].map(ToString::to_string);

        assert_eq!(part_1(lines), 17);
    }
}