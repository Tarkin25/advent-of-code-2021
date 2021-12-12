pub fn part_1<I: IntoIterator<Item=String>>(lines: I, steps: usize) -> usize {
    let mut map: OctopusMap<10, 10> = OctopusMap::from_iter(lines);
    let mut flashes = 0;

    for _ in 0..steps {
        let flashed: usize = map.step()
            .into_iter()
            .map(|row| row.into_iter().filter(|flashed| *flashed).count())
            .sum();

        flashes += flashed;
    }

    flashes
}

pub fn part_2<I: IntoIterator<Item=String>>(lines: I) -> usize {
    let mut map: OctopusMap<10, 10> = OctopusMap::from_iter(lines);

    let mut step = 1;

    loop {
        let flashed = map.step();

        if flashed.into_iter().all(|row| row.into_iter().all(|flashed| flashed)) {
            return step;
        }

        step += 1;
    }
}

const FLASH_LEVEL: u32 = 9;

pub struct OctopusMap<const WIDTH: usize, const HEIGHT: usize> {
    energies: [[u32; WIDTH]; HEIGHT]
}

impl<const WIDTH: usize, const HEIGHT: usize> OctopusMap<WIDTH, HEIGHT> {

    pub fn step(&mut self) -> [[bool; WIDTH]; HEIGHT] {
        self.increase_energy();
        let flashed = self.flash();
        self.reset_energy();

        flashed
    }

    fn flash(&mut self) -> [[bool; WIDTH]; HEIGHT] {
        let mut flashed = [[false; WIDTH]; HEIGHT];

        for row in 0..HEIGHT {
            for column in 0..WIDTH {
                self.flash_rec(row, column, &mut flashed);
            }
        }

        flashed
    }

    fn flash_rec(&mut self, row: usize, column: usize, flashed: &mut [[bool; WIDTH]]) {
        if self.energies[row][column] <= FLASH_LEVEL || *&flashed[row][column] {
            return;
        }

        flashed[row][column] = true;

        for (row, column) in self.adjacent_coordinates(row, column).into_iter().filter_map(|option| option) {
            self.energies[row][column] += 1;

            self.flash_rec(row, column, flashed);
        }
    }

    fn adjacent_coordinates(&self, row: usize, column: usize) -> [Option<(usize, usize)>; 8] {
        let top_left = if row > 0 && column > 0 { Some((row-1, column-1)) } else { None };
        let top = if row > 0 { Some((row-1, column)) } else { None };
        let top_right = if row > 0 && column < WIDTH-1 { Some((row-1, column+1))} else { None };
        let left = if column > 0 { Some((row, column-1))} else { None };
        let right = if column < WIDTH-1 { Some((row, column+1))} else { None };
        let bottom_left = if row < HEIGHT-1 && column > 0 { Some((row+1, column-1))} else { None };
        let bottom = if row < HEIGHT-1 { Some((row+1, column))} else { None };
        let bottom_right = if row < HEIGHT-1 && column < WIDTH-1 { Some((row+1, column+1))} else { None };

        [
            top_left, top, top_right,
            left, right,
            bottom_left, bottom, bottom_right,
        ]
    }

    fn increase_energy(&mut self) {
        for row in &mut self.energies {
            for energy in row {
                *energy += 1;
            }
        }
    }

    fn reset_energy(&mut self) {
        for row in &mut self.energies {
            for energy in row {
                if *energy > FLASH_LEVEL {
                    *energy = 0;
                }
            }
        }
    }

}

impl<const WIDTH: usize, const HEIGHT: usize> FromIterator<String> for OctopusMap<WIDTH, HEIGHT> {
    //noinspection DuplicatedCode
    fn from_iter<T: IntoIterator<Item=String>>(lines: T) -> Self {
        let mut energies = [[0_u32; WIDTH]; HEIGHT];

        for (row_index, row) in lines.into_iter().enumerate() {
            for (column, energy) in row.chars().enumerate() {
                energies[row_index][column] = energy.to_digit(10).unwrap();
            }
        }

        Self { energies }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let lines = [
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ].map(ToString::to_string);

        assert_eq!(part_1(lines.clone(), 10), 204);

        assert_eq!(part_1(lines, 100), 1656);
    }

    #[test]
    fn part_2_works() {
        let lines = [
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ].map(ToString::to_string);

        assert_eq!(part_2(lines), 195);
    }
}