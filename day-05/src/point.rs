use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Sub};
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl PartialEq<(i32, i32)> for Point {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y, }
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const ERROR_MSG: &str = "Expected format [number],[number]";

        fn parse(s: Option<&str>) -> Result<i32, String> {
            s.ok_or(ERROR_MSG.to_string()).and_then(|s| s.parse().map_err(|_| ERROR_MSG.to_string()))
        }

        let mut split = s.split(",");

        let x = parse(split.next())?;
        let y = parse(split.next())?;

        Ok(Self { x, y })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_works() {
        let str = "1,2";

        assert_eq!(Point::from_str(str), Ok(Point { x: 1, y: 2}))
    }
}