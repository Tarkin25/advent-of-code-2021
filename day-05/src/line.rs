use crate::point::Point;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DirectionType {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(Eq, PartialEq, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn direction(&self) -> DirectionType {
        let dx = (self.start.x - self.end.x).abs();
        let dy = (self.start.y - self.end.y).abs();

        if dx == dy {
            DirectionType::Diagonal
        } else if dx == 0 {
            DirectionType::Vertical
        } else if dy == 0 {
            DirectionType::Horizontal
        } else {
            panic!("{} is not vertical", &self)
        }
    }

    pub fn traverse(&self) -> Traverse {
        Traverse::new(self)
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERROR_MSG: &str = "Expected format [number],[number] -> [number],[number]";

        let mut split = s.split(" -> ");

        fn parse(s: Option<&str>) -> Result<Point, String> {
            s.ok_or(ERROR_MSG.to_string()).and_then(FromStr::from_str)
        }

        let start = parse(split.next())?;
        let end = parse(split.next())?;

        Ok(Self { start, end })
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)
    }
}

pub struct Traverse<'a> {
    current: Point,
    target: &'a Point,
    direction: Point,
}

impl<'a> Traverse<'a> {
    fn new(line: &'a Line) -> Self {
        fn normalize(n: i32) -> i32 {
            if n.is_positive() {
                1
            } else if n.is_negative() {
                -1
            } else {
                0
            }
        }

        let dx = normalize(line.end.x - line.start.x);
        let dy = normalize(line.end.y - line.start.y);

        Self {
            current: line.start.clone(),
            target: &line.end,
            direction: (dx, dy).into(),
        }
    }

    fn target_reached(&self) -> bool {
        self.current - self.direction == *self.target
    }
}

impl<'a> Iterator for Traverse<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.target_reached() {
            return None;
        }

        let value = self.current.clone();

        self.current += self.direction;

        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_works() {
        let s = "1,2 -> 3,4";

        assert_eq!(
            Line::from_str(s),
            Ok(Line {
                start: Point { x: 1, y: 2 },
                end: Point { x: 3, y: 4 }
            })
        );
    }

    #[test]
    fn direction_works() {
        let horizontal = Line {
            start: (0, 0).into(),
            end: (9, 0).into(),
        };
        assert_eq!(horizontal.direction(), DirectionType::Horizontal);

        let vertical = Line {
            start: (0, 0).into(),
            end: (0, 9).into(),
        };
        assert_eq!(vertical.direction(), DirectionType::Vertical);

        let diagonal = Line {
            start: (0, 0).into(),
            end: (9, 9).into(),
        };
        assert_eq!(diagonal.direction(), DirectionType::Diagonal);
    }

    #[test]
    fn traverse_works_horizontal() {
        let line = Line {
            start: (0, 0).into(),
            end: (2, 0).into(),
        };
        let mut trav = line.traverse();

        assert_eq!(trav.next(), Some((0, 0).into()));
        assert_eq!(trav.next(), Some((1, 0).into()));
        assert_eq!(trav.next(), Some((2, 0).into()));
        assert_eq!(trav.next(), None);
    }

    #[test]
    fn traverse_works_vertical() {
        let line = Line {
            start: (0, 0).into(),
            end: (0, 2).into(),
        };
        let mut trav = line.traverse();

        assert_eq!(trav.next(), Some((0, 0).into()));
        assert_eq!(trav.next(), Some((0, 1).into()));
        assert_eq!(trav.next(), Some((0, 2).into()));
        assert_eq!(trav.next(), None);
    }

    #[test]
    fn traverse_works_diagonal() {
        let line = Line {
            start: (0, 0).into(),
            end: (2, 2).into(),
        };
        let mut trav = line.traverse();

        assert_eq!(trav.next(), Some((0, 0).into()));
        assert_eq!(trav.next(), Some((1, 1).into()));
        assert_eq!(trav.next(), Some((2, 2).into()));
        assert_eq!(trav.next(), None);
    }
}
