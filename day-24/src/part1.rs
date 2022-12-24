use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use sorted_vec::SortedVec;
use toolkit::map::{Map, Pos};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Open,
    Wall,
    Start,
    Exit,
}

pub fn parse_blizzard(c: char) -> Pos {
    match c {
        '>' => Pos::E,
        '<' => Pos::W,
        '^' => Pos::N,
        'v' => Pos::S,
        _ => unreachable!("Invalid blizzard"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Blizzard {
    pub direction: Pos,
}

pub type BlizzardsMap = HashMap<Pos, Vec<Blizzard>>;

pub fn simulate_blizzard(map: &Map<Tile>, blizzards: &mut BlizzardsMap) {
    let mut new_blizzards = HashMap::new();
    blizzards.iter().for_each(|(pos, current_blizzards)| {
        current_blizzards.iter().for_each(|blizzard| {
            let mut new_pos = *pos + blizzard.direction;
            if !is_in_map(map, &new_pos) {
                new_pos = match blizzard.direction {
                    Pos::N => Pos::new(pos.x, map.bounds.height() - 2),
                    Pos::S => Pos::new(pos.x, 1),
                    Pos::E => Pos::new(1, pos.y),
                    Pos::W => Pos::new(map.bounds.width() - 2, pos.y),
                    _ => panic!("Invalid direction"),
                }
            }

            new_blizzards
                .entry(new_pos)
                .or_insert_with(Vec::new)
                .push(*blizzard);
        });
    });
    *blizzards = new_blizzards;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    pub pos: Pos,
    pub minutes: usize,
    pub target: (bool, Tile),
}

pub fn is_in_map(map: &Map<Tile>, pos: &Pos) -> bool {
    match map.get(*pos) {
        Some(Tile::Wall) => false,
        _ if map.bounds.contains(*pos) => true,
        _ => false,
    }
}

pub fn available_moves(map: &Map<Tile>, blizzards: &BlizzardsMap, player: Player) -> Vec<Pos> {
    let mut moves = Vec::new();
    for direction in [Pos::ZERO, Pos::N, Pos::E, Pos::S, Pos::W] {
        let next_pos = player.pos + direction;
        if is_in_map(map, &next_pos) && !blizzards.contains_key(&next_pos) {
            moves.push(next_pos);
        }
    }
    moves
}

pub fn parse_blizzard_map(input: &str) -> (Map<Tile>, BlizzardsMap) {
    let mut blizzards = HashMap::new();
    let mut map = Map::parse(input, |c, x, y| match c {
        '#' => Some(Tile::Wall),
        '.' if y == 0 => Some(Tile::Start),
        '.' => Some(Tile::Open),
        _ => {
            blizzards.insert(
                Pos::new(x, y),
                vec![Blizzard {
                    direction: parse_blizzard(c),
                }],
            );
            None
        }
        _ => None,
    });

    map.tiles
        .entry(Pos::new(map.bounds.width() - 2, map.bounds.height() - 1))
        .and_modify(|t| *t = Tile::Exit);

    (map, blizzards)
}

pub fn navigate_blizzards(input: &str) -> Option<usize> {
    let (mut map, mut blizzards) = parse_blizzard_map(input);

    let loop_count = ((map.bounds.width() - 2) * (map.bounds.height() - 2)) as usize;
    let mut blizzards_history = Vec::with_capacity(loop_count);
    for _ in 0..loop_count {
        blizzards_history.push(blizzards.clone());
        simulate_blizzard(&map, &mut blizzards);
    }

    let mut paths: VecDeque<Player> = VecDeque::new();
    paths.push_back(Player {
        pos: Pos::new(1, 0), // Start
        minutes: 0,
        target: (false, Tile::Exit),
    });

    let mut visited = HashSet::new();
    visited.insert((paths[0].pos, paths[0].minutes % loop_count));

    while let Some(player) = paths.pop_front() {
        if map.tiles.get(&player.pos) == Some(&Tile::Exit) {
            return Some(player.minutes);
        }

        let next_minutes = player.minutes + 1;
        let next_index = next_minutes % loop_count;

        let blizzard_turn = &blizzards_history[next_index];

        for next_pos in available_moves(&map, blizzard_turn, player) {
            if !visited.contains(&(next_pos, next_index)) {
                visited.insert((next_pos, next_index));
                paths.push_back(Player {
                    pos: next_pos,
                    minutes: next_minutes,
                    target: player.target,
                });
            }
        }
    }

    None
}

pub fn part1() {}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(navigate_blizzards(input), Some(18));
    }
}
