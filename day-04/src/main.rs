mod board;

use crate::board::Board;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

fn parse_boards(lines: impl Iterator<Item = String>) -> Result<Vec<Board>, String> {
    let mut lines = lines.filter(|line| !line.is_empty()).peekable();
    let mut boards = vec![];

    while let Some(_) = lines.peek() {
        let board = Board::try_from(&mut lines)?;
        boards.push(board);
    }

    Ok(boards)
}

fn parse_numbers(lines: &mut impl Iterator<Item = String>) -> Result<Vec<u8>, ParseIntError> {
    lines.next().unwrap().split(",").map(str::parse).collect()
}

struct GameResult {
    winning_board: Board,
    winning_number: u8,
}

fn get_winning_board(drawn_numbers: Vec<u8>, mut boards: Vec<Board>) -> Option<GameResult> {
    for number in drawn_numbers {
        for i in 0..boards.len() {
            if boards[i].mark_field_and_check_if_won(number) {
                return Some(GameResult {
                    winning_board: boards.remove(i),
                    winning_number: number,
                });
            }
        }
    }

    None
}

fn get_score(mut lines: impl Iterator<Item = String>) -> u32 {
    let drawn_numbers = parse_numbers(&mut lines).unwrap();
    let boards = parse_boards(lines).unwrap();
    let GameResult {
        winning_board,
        winning_number,
    } = get_winning_board(drawn_numbers, boards).expect("No winning board found");
    let sum_of_unmarked: u32 = winning_board
        .rows
        .iter()
        .map(|row| row.iter().filter(|field| !field.marked).map(|field| field.number as u32).sum::<u32>())
        .sum();

    sum_of_unmarked * (winning_number as u32)
}

fn main() {
    let lines = File::open("./input.txt")
        .map(BufReader::new)
        .unwrap()
        .lines()
        .map(|line| line.unwrap());
    let score = get_score(lines);

    println!("{}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_boards_works() {
        let lines = vec![
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ]
        .into_iter()
        .map(ToString::to_string);

        let boards = parse_boards(lines);
        assert!(boards.is_ok());
        assert_eq!(2, boards.unwrap().len());
    }

    #[test]
    fn get_score_works() {
        let lines = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ]
        .into_iter()
        .map(ToString::to_string);

        let score = get_score(lines);

        assert_eq!(score, 4512);
    }
}
