mod cave;

use crate::cave::CaveSystem;

pub fn part_1(lines: impl IntoIterator<Item = String>) -> usize {
    let cave_system = CaveSystem::from_iter(lines);

    cave_system.paths_between("start", "end").len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works() {
        let lines = [
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end",
        ].map(ToString::to_string);

        assert_eq!(part_1(lines), 10);
    }
}