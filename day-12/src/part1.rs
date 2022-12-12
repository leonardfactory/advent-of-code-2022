use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
}

pub struct MapGraph {
    pub map: HashMap<Pos, usize>,
    pub start: Pos,
    pub end: Pos,
}

impl MapGraph {
    pub fn parse(input: &str) -> Self {
        let mut map = HashMap::new();
        let mut start = Pos::new(0, 0);
        let mut end = Pos::new(0, 0);
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let pos = Pos::new(x as i32, y as i32);
                let height = match c {
                    'S' => {
                        start = pos;
                        0
                    }
                    'E' => {
                        end = pos;
                        'z' as usize - 'a' as usize
                    }
                    c => c as usize - 'a' as usize,
                };
                map.insert(pos, height);
            });
        });

        Self { map, start, end }
    }

    pub fn visitable_neighbours(&self, pos: &Pos) -> Vec<(Pos, usize)> {
        vec![
            Pos::new(pos.x, pos.y - 1),
            Pos::new(pos.x, pos.y + 1),
            Pos::new(pos.x - 1, pos.y),
            Pos::new(pos.x + 1, pos.y),
        ]
        .iter()
        .filter_map(|p| self.map.get(p).map(|height| (*p, *height)))
        .filter(|(_, height)| *height <= self.map[pos] + 1)
        .collect_vec()
    }

    pub fn descendable_neighbours(&self, pos: &Pos) -> Vec<(Pos, usize)> {
        vec![
            Pos::new(pos.x, pos.y - 1),
            Pos::new(pos.x, pos.y + 1),
            Pos::new(pos.x - 1, pos.y),
            Pos::new(pos.x + 1, pos.y),
        ]
        .iter()
        .filter_map(|p| self.map.get(p).map(|height| (*p, *height)))
        .filter(|(_, height)| *height >= self.map[pos] - 1)
        .collect_vec()
    }
}

pub fn shortest_path(input: &str) -> Option<usize> {
    let graph = MapGraph::parse(input);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((graph.start, 0));
    visited.insert(graph.start);

    while let Some((pos, len)) = queue.pop_front() {
        // println!("Visiting {:?} (len: {}) end={:?}", pos, len, graph.end);
        if pos == graph.end {
            return Some(len);
        }

        for (neighbour, _) in graph.visitable_neighbours(&pos) {
            if !visited.contains(&neighbour) {
                visited.insert(neighbour);
                queue.push_back((neighbour, len + 1));
            }
        }
    }

    None
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(shortest_path(input), Some(31));
    }
}
