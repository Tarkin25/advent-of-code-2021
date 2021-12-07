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

fn parse_numbers(lines: &mut impl Iterator<Item = String>) -> Result<Vec<u32>, ParseIntError> {
    lines.next().unwrap().split(",").map(str::parse).collect()
}

#[derive(Debug)]
struct GameResult {
    winning_board: Board,
    winning_number: u32,
}

fn get_last_winning_board(drawn_numbers: Vec<u32>, mut boards: Vec<Board>) -> GameResult {
    let mut drawn_numbers = drawn_numbers.into_iter();

    while let Some(number) = drawn_numbers.next() {
        for board in &mut boards {
            board.mark_field(number);
        }

        boards.retain(|board| !board.has_won());

        if boards.len() == 1 {
            break;
        }
    }

    while let Some(number) = drawn_numbers.next() {
        let board = &mut boards[0];
        board.mark_field(number);

        if board.has_won() {
            let board = boards.remove(0);

            let result = GameResult {
                winning_board: board,
                winning_number: number,
            };

            println!("{:?}", &result);

            return result;
        }
    }

    unreachable!("You dun fucked up!")
}

fn get_score(mut lines: impl Iterator<Item = String>) -> u32 {
    let drawn_numbers = parse_numbers(&mut lines).unwrap();
    let boards = parse_boards(lines).unwrap();
    let GameResult {
        winning_board,
        winning_number,
    } = get_last_winning_board(drawn_numbers, boards);

    winning_board.sum_of_unmarked() * (winning_number as u32)
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

        assert_eq!(score, 1924);
    }

    #[test]
    fn get_last_winning_board_works() {
        let mut lines = vec![
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

        let drawn_numbers = parse_numbers(&mut lines).unwrap();
        let boards = parse_boards(lines).unwrap();

        let GameResult { winning_number, winning_board } = get_last_winning_board(drawn_numbers, boards);

        assert_eq!(winning_number, 13);
        assert_eq!(winning_board.sum_of_unmarked(), 148);

    }
}
