mod cave;

use crate::cave::CaveSystem;

pub fn part_2(lines: impl IntoIterator<Item = String>) -> usize {
    let cave_system = CaveSystem::from_iter(lines);

    let paths = cave_system.paths_between("start", "end");

    for path in &paths {
        let path = path.into_iter().map(|id| id.name(&cave_system)).collect::<Vec<_>>();
        println!("{:?}", path);
    }

    paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2_works() {
        let lines = [
            "start-A",
            "start-b",
            "A-c",
            "A-b",
            "b-d",
            "A-end",
            "b-end",
        ].map(ToString::to_string);

        assert_eq!(part_2(lines), 36);
    }
}