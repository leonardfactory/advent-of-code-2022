use std::{collections::HashMap, fmt::Display};

use colored::Colorize;
use toolkit::map::{Map as BaseMap, Pos, TileDisplay};

type Map = BaseMap<Tile>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Elf,
}

impl TileDisplay for Tile {
    fn map_print(&self) -> Box<dyn Display> {
        match self {
            Tile::Empty => Box::new(".".white().on_black()),
            Tile::Elf => Box::new("#".green()),
        }
    }
}

pub fn parse_map(input: &str) -> Map {
    Map::parse(input, |c, _, _| match c {
        '.' => None,
        '#' => Some(Tile::Elf),
        _ => panic!("Unknown tile: {}", c),
    })
}

fn are_occupied(map: &Map, pos: Pos, diffs: &[Pos]) -> bool {
    diffs.iter().any(|&d| map.get(pos + d).is_some())
}

const DIRECTIONS: &[(&[Pos; 3], Pos); 4] = &[
    (&[Pos::N, Pos::NE, Pos::NW], Pos::N),
    (&[Pos::S, Pos::SE, Pos::SW], Pos::S),
    (&[Pos::W, Pos::NW, Pos::SW], Pos::W),
    (&[Pos::E, Pos::NE, Pos::SE], Pos::E),
];

fn elf_decision(map: &Map, pos: Pos, round: usize) -> Option<Pos> {
    if map.all_neighbors(pos).is_empty() {
        return None;
    }

    for i in 0..4 {
        let (diffs, dir) = DIRECTIONS[(i + round) % 4];
        if !are_occupied(map, pos, diffs) {
            return Some(pos + dir);
        }
    }
    None
}

pub fn simulate_step(map: &mut Map, round: usize) {
    let mut targets = HashMap::new();
    let decisions: Vec<_> = map
        .tiles
        .iter()
        .map(|(&pos, _tile)| {
            let decision = elf_decision(map, pos, round);
            if let Some(decision_pos) = decision {
                targets
                    .entry(decision_pos)
                    .and_modify(|x| *x += 1)
                    .or_insert(1);
            }
            (pos, decision)
        })
        .collect();

    let decisions: HashMap<_, _> = decisions
        .iter()
        .map(|(pos, decision)| match decision {
            None => (*pos, Tile::Elf),
            Some(decision) if targets[decision] >= 2 => (*pos, Tile::Elf),
            Some(decision) => (*decision, Tile::Elf),
        })
        .collect();

    map.tiles = decisions;
    map.update_bounds();
}

pub fn simulate_elves(input: &str) -> i32 {
    let mut map = parse_map(input);

    for i in 0..10 {
        simulate_step(&mut map, i);
    }

    (map.bounds.width() * map.bounds.height()) - map.tiles.len() as i32
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example_elves() {
        let input = include_str!("../test.txt");
        assert_eq!(simulate_elves(input), 110);
    }
}
