fn main() {
    let sum = part_1::<_, 100, 100>(input::lines!());
    println!("Part 1: {}", sum);

    let product = part_2::<_, 100, 100>(input::lines!());
    println!("Part 2: {}", product);
}

fn part_1<I: IntoIterator<Item=String>, const WIDTH: usize, const HEIGHT: usize>(lines: I) -> u32 {
    let map = parse_map::<I, WIDTH, HEIGHT>(lines);
    let mut risk_factor_sum = 0_u32;

    let smaller_than_adjacent = smaller_than_adjacent(&map);

    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if smaller_than_adjacent(row, column) {
                risk_factor_sum += map[row][column] + 1;
            }
        }
    }

    risk_factor_sum
}

fn part_2<I: IntoIterator<Item = String>, const WIDTH: usize, const HEIGHT: usize>(lines: I) -> u32 {
    let height_map = parse_map::<I, WIDTH, HEIGHT>(lines);
    let mut low_points = vec![];

    let smaller_than_adjacent = smaller_than_adjacent(&height_map);

    for row in 0..height_map.len() {
        for column in 0..height_map[row].len() {
            if smaller_than_adjacent(row, column) {
                low_points.push((row, column));
            }
        }
    }

    let mut basin_map = [[false; WIDTH]; HEIGHT];

    let mut basin_sizes = low_points.into_iter()
        .map(|(row, column)| basin_size(&height_map, &mut basin_map, row, column))
        .collect::<Vec<_>>();

    basin_sizes.sort();

    basin_sizes.into_iter().rev().take(3).product()
}

fn basin_size<const WIDTH: usize>(height_map: &[[u32; WIDTH]], basin_map: &mut [[bool; WIDTH]], row: usize, column: usize) -> u32 {
    let point = height_map[row][column];

    if point == 9 {
        return 0;
    }

    let mut size = 1;
    basin_map[row][column] = true;

    let is_part_of_basin = |row: usize, column: usize, basin_map: &[[bool; WIDTH]]| {
        !basin_map[row][column] && point < height_map[row][column]
    };

    let mut add_to_size = |row: usize, column: usize, basin_map: &mut [[bool; WIDTH]]| {
        size += basin_size::<WIDTH>(height_map, basin_map, row, column)
    };

    // top
    if row > 0 && is_part_of_basin(row-1, column, basin_map) {
        add_to_size(row-1, column, basin_map);
    }
    // bottom
    if row < height_map.len()-1 && is_part_of_basin(row+1, column, basin_map) {
        add_to_size(row+1, column, basin_map);
    }
    // left
    if column > 0 && is_part_of_basin(row, column-1, basin_map) {
        add_to_size(row, column-1, basin_map);
    }
    // right
    if column < height_map[row].len()-1 && is_part_of_basin(row, column+1, basin_map) {
        add_to_size(row, column+1, basin_map);
    }

    size
}

fn smaller_than_adjacent<const WIDTH: usize>(map: &[[u32; WIDTH]]) -> impl Fn(usize, usize) -> bool + '_ {
    |row: usize, column: usize| {
        let point = map[row][column];
        let top = row == 0 || point < map[row-1][column];
        let bottom = row == map.len()-1 || point < map[row+1][column];
        let left = column == 0 || point < map[row][column-1];
        let right = column == map[row].len()-1 || point < map[row][column+1];

        top && bottom && left && right
    }
}

fn parse_map<I: IntoIterator<Item = String>, const WIDTH: usize, const HEIGHT: usize>(lines: I) -> [[u32; WIDTH]; HEIGHT] {
    let mut map = [[0_u32; WIDTH]; HEIGHT];

    for (row, line) in lines.into_iter().enumerate() {
        for (column, c) in line.chars().enumerate() {
            map[row][column] = c.to_digit(10).unwrap();
        }
    }

    map
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

    #[test]
    fn part_2_works() {
        let lines = [
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ].map(ToString::to_string);

        let product = part_2::<_, 10, 5>(lines);
        assert_eq!(product, 1134);
    }
}
