const PARENS_OPEN: char = '(';
const PARENS_CLOSE: char = ')';
const BRACKET_OPEN: char = '[';
const BRACKET_CLOSE: char = ']';
const BRACE_OPEN: char = '{';
const BRACE_CLOSE: char = '}';
const ANGLE_OPEN: char = '<';
const ANGLE_CLOSE: char = '>';

fn main() {
    let error_score = part_1(input::lines!());
    println!("Part 1: {}", error_score);

    let error_score = part_2(input::lines!());
    println!("Part 2: {}", error_score);
}

fn part_2(lines: impl IntoIterator<Item = String>) -> usize {
    let mut scores = lines
        .into_iter()
        .filter(|line| first_illegal_character(&line).is_none())
        .map(|line| autocomplete(&line))
        .map(|autocomplete| calculate_score(&autocomplete))
        .collect::<Vec<_>>();

    scores.sort();

    scores[scores.len()/2]
}

fn part_1(lines: impl IntoIterator<Item = String>) -> usize {
    lines
        .into_iter()
        .map(|line| first_illegal_character(&line))
        .map(|char| {
            if let Some(char) = char {
                match char {
                    PARENS_CLOSE => 3_usize,
                    BRACKET_CLOSE => 57_usize,
                    BRACE_CLOSE => 1197_usize,
                    ANGLE_CLOSE => 25137_usize,
                    _ => unreachable!()
                }
            } else {
                0_usize
            }
        })
        .sum()
}

macro_rules! check_popped {
    ($expected:ident, $popped:expr, $given:ident) => {
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
            PARENS_OPEN | BRACE_OPEN | BRACKET_OPEN | ANGLE_OPEN => stack.push(char),
            PARENS_CLOSE => {
                check_popped!(PARENS_OPEN, stack.pop(), char);
            },
            BRACE_CLOSE => {
                check_popped!(BRACE_OPEN, stack.pop(), char);
            },
            BRACKET_CLOSE => {
                check_popped!(BRACKET_OPEN, stack.pop(), char);
            },
            ANGLE_CLOSE => {
                check_popped!(ANGLE_OPEN, stack.pop(), char);
            },
            invalid => return Some(invalid)
        }
    }

    None
}

fn autocomplete(line: &str) -> String {
    let mut stack = Vec::with_capacity(line.len());

    for char in line.chars() {
        match char {
            PARENS_OPEN | BRACE_OPEN | BRACKET_OPEN | ANGLE_OPEN => {
                stack.push(char);
            },
            PARENS_CLOSE | BRACE_CLOSE | BRACKET_CLOSE | ANGLE_CLOSE => {
                stack.pop();
            },
            _ => unreachable!()
        }
    }

    let mut autocomplete = String::with_capacity(stack.len());

    while let Some(popped) = stack.pop() {
        match popped {
            PARENS_OPEN => autocomplete.push(PARENS_CLOSE),
            BRACE_OPEN => autocomplete.push(BRACE_CLOSE),
            BRACKET_OPEN => autocomplete.push(BRACKET_CLOSE),
            ANGLE_OPEN => autocomplete.push(ANGLE_CLOSE),
            _ => unreachable!()
        }
    }

    autocomplete
}

fn calculate_score(autocomplete: &str) -> usize {
    autocomplete.chars()
        .map(|char| {
            match char {
                PARENS_CLOSE => 1_usize,
                BRACKET_CLOSE => 2_usize,
                BRACE_CLOSE => 3_usize,
                ANGLE_CLOSE => 4_usize,
                _ => unreachable!()
            }
        })
        .fold(0, |total, current| total * 5 + current)
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
    fn autocomplete_works() {
        let line = "[({(<(())[]>[[{[]{<()<>>";
        assert_eq!(&autocomplete(line), "}}]])})]");

        let line = "[(()[<>])]({[<{<<[]>>(";
        assert_eq!(&autocomplete(line), ")}>]})");

        let line = "(((({<>}<{<{<>}{[]{[]{}";
        assert_eq!(&autocomplete(line), "}}>}>))))");

        let line = "{<[[]]>}<{[{[{[]{()[[[]";
        assert_eq!(&autocomplete(line), "]]}}]}]}>");

        let line = "<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(&autocomplete(line), "])}>");
    }

    #[test]
    fn calculate_score_works() {
        fn check_score(autocomplete: &str, expected: usize) {
            assert_eq!(calculate_score(autocomplete), expected);
        }

        check_score("}}]])})]", 288957);
        check_score(")}>]})", 5566);
        check_score("}}>}>))))", 1480781);
        check_score("]]}}]}]}>", 995444);
        check_score("])}>", 294);
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

    #[test]
    fn part_2_works() {
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

        let error_score = part_2(lines);
        assert_eq!(error_score, 288957);
    }
}
