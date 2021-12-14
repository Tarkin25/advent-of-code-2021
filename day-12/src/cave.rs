use std::collections::{HashMap, HashSet};
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NodeId(usize);

impl NodeId {

    fn connect(self, other: NodeId, cs: &mut CaveSystem) {
        cs[self].connected_caves.insert(other);
        cs[other].connected_caves.insert(self);
    }

    fn available_caves<'a>(self, cs: &'a CaveSystem, visited: &'a [NodeId], visited_twice: &'a Option<NodeId>) -> impl Iterator<Item=NodeId> + 'a {
        cs[self].connected_caves
            .iter()
            .filter(|cave| {
                cave.is_big(cs) ||
                    visited_twice.is_none() ||
                    !visited.contains(*cave)
            })
            .copied()
    }

    fn is_big(self, cs: &CaveSystem) -> bool {
        *&cs[self].big
    }

    pub fn name(self, cs: &CaveSystem) -> &str {
        &cs[self].name
    }

}

#[derive(Debug)]
pub struct Cave {
    id: NodeId,
    name: String,
    big: bool,
    connected_caves: HashSet<NodeId>,
}

impl Cave {

    fn new(name: String, id: NodeId) -> Self {
        let big = name.chars().next().unwrap().is_uppercase();

        Self {
            name,
            big,
            connected_caves: HashSet::new(),
            id,
        }
    }
}

#[derive(Default, Debug)]
pub struct CaveSystem {
    caves: Vec<Cave>,
    by_name: HashMap<String, NodeId>,
}

impl CaveSystem {

    pub fn paths_between(&self, start: &str, end: &str) -> Vec<Vec<NodeId>> {
        let start = self.node_id(start);
        let end = self.node_id(end);
        let current_path = vec![];

        self.paths_to(end, start, current_path, None)
    }

    fn paths_to(&self, end: NodeId, current: NodeId, current_path: Vec<NodeId>, visited_twice: Option<NodeId>) -> Vec<Vec<NodeId>> {
        let mut paths = vec![];

        if current.available_caves(self, &current_path, &visited_twice).any(|cave| cave == end) {
            let mut path = current_path.clone();
            path.push(current);
            path.push(end);

            paths.push(path);
        }

        for connected_cave in current.available_caves(self, &current_path, &visited_twice) {
            let visited_twice = if visited_twice.is_none() {
                if current_path.contains(&connected_cave) {
                    Some(connected_cave)
                } else {
                    None
                }
            } else {
                visited_twice
            };

            let mut current_path = current_path.clone();
            current_path.push(current);

            paths.append(&mut self.paths_to(end, connected_cave, current_path, visited_twice));
        }

        paths
    }

    fn connect(&mut self, a: String, b: String) {
        let a = self.or_insert(a);
        let b = self.or_insert(b);

        a.connect(b, self);
    }

    fn or_insert(&mut self, name: String) -> NodeId {
        if let Some(id) = self.by_name.get(&name) {
            id.clone()
        } else {
            let index = self.caves.len();
            let id = NodeId(index);
            self.caves.push(Cave::new(name.clone(), id));
            self.caves[index].id = id;
            self.by_name.insert(name, id);

            id
        }
    }

    fn node_id(&self, name: &str) -> NodeId {
        self.by_name[name]
    }

}

impl FromIterator<String> for CaveSystem {
    fn from_iter<T: IntoIterator<Item=String>>(lines: T) -> Self {
        let mut this = Self::default();

        for line in lines {
            let mut split = line.split("-");
            let first_key = split.next().unwrap().to_string();
            let second_key = split.next().unwrap().to_string();

            this.connect(first_key, second_key);
        }

        this
    }
}

impl Index<NodeId> for CaveSystem {
    type Output = Cave;

    fn index(&self, index: NodeId) -> &Self::Output {
        &self.caves[index.0]
    }
}

impl IndexMut<NodeId> for CaveSystem {
    fn index_mut(&mut self, index: NodeId) -> &mut Self::Output {
        &mut self.caves[index.0]
    }
}