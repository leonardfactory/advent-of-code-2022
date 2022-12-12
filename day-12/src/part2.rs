use std::collections::{HashSet, VecDeque};

use crate::part1::MapGraph;

pub fn shortest_path_from_anywhere(input: &str) -> Option<usize> {
    let graph = MapGraph::parse(input);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((graph.end, 0));
    visited.insert(graph.end);

    while let Some((pos, len)) = queue.pop_front() {
        // println!("Visiting {:?} (len: {}) end={:?}", pos, len, graph.end);
        if graph.map[&pos] == 0 {
            return Some(len);
        }

        for (neighbour, _) in graph.descendable_neighbours(&pos) {
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
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(shortest_path_from_anywhere(input), Some(29));
    }
}
