fn main() {
    let error_score = part_1(input::lines!());
    println!("Part 1: {}", error_score);
}

fn part_1(lines: impl IntoIterator<Item = String>) -> usize {
    lines
        .into_iter()
        .map(|line| first_illegal_character(&line))
        .map(|char| {
            if let Some(char) = char {
                match char {
                    ')' => 3_usize,
                    ']' => 57_usize,
                    '}' => 1197_usize,
                    '>' => 25137_usize,
                    _ => unreachable!()
                }
            } else {
                0_usize
            }
        })
        .sum()
}

macro_rules! check_popped {
    ($expected:literal, $popped:expr, $given:ident) => {
        if let Some(popped) = $popped {
            if popped != $expected {
                return Some($given);
            }
        } else {
            return Some($given);
        }
    };
}

fn first_illegal_character(line: &str) -> Option<char> {
    let mut stack = Vec::with_capacity(line.len());

    for char in line.chars() {
        match char {
            '(' | '{' | '[' | '<' => stack.push(char),
            ')' => {
                check_popped!('(', stack.pop(), char);
            },
            '}' => {
                check_popped!('{', stack.pop(), char);
            },
            ']' => {
                check_popped!('[', stack.pop(), char);
            },
            '>' => {
                check_popped!('<', stack.pop(), char);
            },
            invalid => panic!("unexpected character \"{}\"", invalid)
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_illegal_character_works() {
        let line = "{([(<{}[<>[]}>{[]{[(<()>";
        assert_eq!(first_illegal_character(line), Some('}'));
    }

    #[test]
    fn part_1_works() {
        let lines = [
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ].map(ToString::to_string);

        let error_score = part_1(lines);
        assert_eq!(error_score, 26397);
    }
}
