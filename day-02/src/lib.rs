use std::str::FromStr;

pub enum Direction {
    Up,
    Down,
    Forward,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            variant => Err(format!("Invalid direction {}", variant))
        }
    }
}

pub struct Instruction {
    pub direction: Direction,
    pub amount: u32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");

        let direction = split.next().ok_or("Expected direction".to_string())?;
        let amount = split.next().ok_or("Expected amount".to_string())?;

        let instruction = Self {
            direction: direction.parse()?,
            amount: amount.parse().map_err(|_| "ParseIntError".to_string())?,
        };

        Ok(instruction)
    }
}

pub fn parse_instructions(lines: impl IntoIterator<Item = String>) -> impl Iterator<Item = Result<Instruction, String>> {
    lines.into_iter().map(|line| line.parse())
}

pub struct Position {
    pub horizontal_position: u32,
    pub depth: u32,
}

impl FromIterator<Instruction> for Position {
    fn from_iter<T: IntoIterator<Item=Instruction>>(iter: T) -> Self {
        let mut horizontal_position = 0_u32;
        let mut depth = 0_u32;

        for instruction in iter {
            let Instruction { direction, amount } = instruction;

            match direction {
                Direction::Up => depth -= amount,
                Direction::Down => depth += amount,
                Direction::Forward => horizontal_position += amount,
            }
        }

        Self {
            horizontal_position,
            depth,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = vec![
            "forward 5".to_string(),
            "down 5".to_string(),
            "forward 8".to_string(),
            "up 3".to_string(),
            "down 8".to_string(),
            "forward 2".to_string(),
        ];

        let position: Position = parse_instructions(lines)
            .map(|instruction| instruction.unwrap())
            .collect();

        assert_eq!(position.horizontal_position, 15);
        assert_eq!(position.depth, 10);
    }
}
