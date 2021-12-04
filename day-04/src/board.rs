const BOARD_SIZE: usize = 5;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Field {
    pub number: u8,
    pub marked: bool,
}

impl Field {
    pub fn new(number: u8) -> Self {
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
                let number: u8 = number
                    .parse()
                    .map_err(|_| "Expected field to be a number".to_string())?;

                columns[x] = Some(Field::new(number));
            }

            rows[y] = columns;
        }

        let rows = rows.map(|row| row.map(Option::unwrap));

        Ok(Self { rows })
    }

    pub fn mark_field_and_check_if_won(&mut self, number: u8) -> bool {
        for row in 0..BOARD_SIZE {
            for column in 0..BOARD_SIZE {
                let board = &mut self.rows[row][column];

                if board.number == number {
                    board.marked = true;

                    if self.check_if_row_won(row) || self.check_if_column_won(column) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn check_if_row_won(&self, row: usize) -> bool {
        self.rows[row].iter().all(|field| field.marked)
    }

    fn check_if_column_won(&self, column: usize) -> bool {
        for row in &self.rows {
            if !row[column].marked {
                return false;
            }
        }

        true
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
    fn check_if_row_won() {
        let board = Board {
            rows: [
                [MARKED, MARKED, MARKED, MARKED, MARKED],
                [UNMARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
                [UNMARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
                [UNMARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
                [UNMARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
            ],
        };

        assert!(board.check_if_row_won(0));
    }

    #[test]
    fn check_if_column_won() {
        let board = Board {
            rows: [
                [MARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
                [MARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
                [MARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
                [MARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
                [MARKED, UNMARKED, UNMARKED, UNMARKED, UNMARKED],
            ],
        };

        assert!(board.check_if_column_won(0));
    }

    #[test]
    fn mark_field_and_check_if_won_with_empty_board() {
        let mut board = Board::default();
        board.rows[0][0].number = 1;

        let won = board.mark_field_and_check_if_won(1);
        assert!(board.rows[0][0].marked);
        assert!(!won);
    }

    #[test]
    fn mark_field_and_check_if_won_with_winning_board() {
        let mut board = Board {
            rows: [
                [UNMARKED, MARKED, MARKED, MARKED, MARKED],
                [UNMARKED; 5],
                [UNMARKED; 5],
                [UNMARKED; 5],
                [UNMARKED; 5],
            ],
        };
        board.rows[0][0].number = 1;

        let won = board.mark_field_and_check_if_won(1);
        assert!(board.rows[0][0].marked);
        assert!(won);
    }
}
