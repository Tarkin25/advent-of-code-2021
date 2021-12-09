fn main() {
    let sum = part_1::<_, 100, 100>(input::lines!());
    println!("Part 1: {}", sum);
}

fn part_1<I: IntoIterator<Item=String>, const WIDTH: usize, const HEIGHT: usize>(lines: I) -> u32 {
    let mut map = [[0_u32; WIDTH]; HEIGHT];

    for (row, line) in lines.into_iter().enumerate() {
        for (column, c) in line.chars().enumerate() {
            map[row][column] = c.to_digit(10).unwrap();
        }
    }

    let mut risk_factor_sum = 0_u32;

    for row in 0..map.len() {
        for column in 0..map[row].len() {
            let point = map[row][column];
            let top = row == 0 || point < map[row - 1][column];
            let bottom = row == map.len() - 1 || point < map[row + 1][column];
            let left = column == 0 || point < map[row][column - 1];
            let right = column == map[row].len() - 1 || point < map[row][column + 1];

            if top && bottom && left && right {
                risk_factor_sum += point + 1;
            }
        }
    }

    risk_factor_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let lines = [
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ].map(ToString::to_string);

        let sum = part_1::<_, 10, 5>(lines);
        assert_eq!(sum, 15);
    }
}
