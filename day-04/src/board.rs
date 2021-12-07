const BOARD_SIZE: usize = 5;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Field {
    pub number: u32,
    pub marked: bool,
}

impl Field {
    pub fn new(number: u32) -> Self {
        Self {
            number,
            marked: false,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pub rows: [[Field; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    pub fn try_from<I: Iterator<Item = String>>(lines: &mut I) -> Result<Self, String> {
        let mut rows: [[Option<Field>; BOARD_SIZE]; BOARD_SIZE] = [[None; BOARD_SIZE]; BOARD_SIZE];

        for y in 0..BOARD_SIZE {
            let mut columns = rows[y];
            let line = lines.next().ok_or(format!("Expected line {}", y))?;
            let mut line = line.split_whitespace();

            for x in 0..BOARD_SIZE {
                let number = line.next().ok_or(format!("Expected field {}", x))?;
                let number: u32 = number
                    .parse()
                    .map_err(|_| "Expected field to be a number".to_string())?;

                columns[x] = Some(Field::new(number));
            }

            rows[y] = columns;
        }

        let rows = rows.map(|row| row.map(Option::unwrap));

        Ok(Self { rows })
    }

    pub fn mark_field(&mut self, number: u32) {
        for row in &mut self.rows {
            for field in row {
                if field.number == number {
                    field.marked = true;
                }
            }
        }
    }

    pub fn has_won(&self) -> bool {
        if self.rows.iter().any(|row| row.iter().all(|field| field.marked)) {
            return true;
        }

        for i in 0..BOARD_SIZE {
            if self.has_won_column(i) {
                return true;
            }
        }

        false
    }

    fn has_won_column(&self, column: usize) -> bool {
        for row in &self.rows {
            if !row[column].marked {
                return false;
            }
        }

        true
    }

    pub fn sum_of_unmarked(&self) -> u32 {
        self.rows.iter()
            .map(|row| row.iter().filter(|field| !field.marked).map(|field| field.number as u32).sum::<u32>())
            .sum()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            rows: [[Field {
                number: 0,
                marked: false,
            }; BOARD_SIZE]; BOARD_SIZE],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MARKED: Field = Field {
        number: 0,
        marked: true,
    };
    const UNMARKED: Field = Field {
        number: 0,
        marked: false,
    };

    #[test]
    fn try_from_works() {
        let mut lines = vec![
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ]
        .into_iter()
        .map(ToString::to_string);

        let board = Board::try_from(&mut lines);
        assert!(board.is_ok());

        let expected_rows = [
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ]
        .map(|row| row.map(Field::new));

        assert_eq!(expected_rows, board.unwrap().rows);
    }

    #[test]
    fn has_won_when_won_by_row() {
        let board = Board {
            rows: [
                [MARKED, MARKED, MARKED, MARKED, MARKED],
                [UNMARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
                [UNMARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
                [UNMARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
                [UNMARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
            ],
        };

        assert!(board.has_won());
    }

    #[test]
    fn has_won_when_won_by_column() {
        let board = Board {
            rows: [
                [MARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
                [MARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
                [MARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
                [MARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
                [MARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
            ],
        };

        assert!(board.has_won());
    }

    #[test]
    fn mark_field() {
        let mut board = Board::default();
        board.rows[0][0].number = 1;
        board.mark_field(1);

        assert!(board.rows[0][0].marked);
    }
}
