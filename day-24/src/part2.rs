use std::collections::{HashMap, HashSet, VecDeque};

use toolkit::map::{Map, Pos};

use crate::part1::{available_moves, parse_blizzard_map, simulate_blizzard, Player, Tile};

impl Player {
    pub fn is_finished(&self) -> bool {
        self.target.0 && self.target.1 == Tile::Exit
    }

    pub fn next_target(&self, current: Option<&Tile>) -> Option<(bool, Tile)> {
        match (self.target, current) {
            ((false, Tile::Exit), Some(&Tile::Exit)) => Some((false, Tile::Start)),
            ((false, Tile::Start), Some(&Tile::Start)) => Some((true, Tile::Exit)),
            ((true, Tile::Exit), Some(&Tile::Exit)) => None,
            _ => Some(self.target),
        }
    }
}

pub fn navigate_with_snacks(input: &str) -> Option<usize> {
    let (map, mut blizzards) = parse_blizzard_map(input);

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
        let next_target = player.next_target(map.tiles.get(&player.pos));
        if next_target.is_none() {
            return Some(player.minutes);
        } else if next_target != Some(player.target) {
            paths.clear();
            visited.clear();
        }

        // println!("{}: {:?} {:?}", player.minutes, player.target, player.pos);

        let next_minutes = player.minutes + 1;
        let next_index = next_minutes % loop_count;

        let blizzard_turn = &blizzards_history[next_index];

        for next_pos in available_moves(&map, blizzard_turn, player) {
            if !visited.contains(&(next_pos, next_index)) {
                visited.insert((next_pos, next_index));
                paths.push_back(Player {
                    pos: next_pos,
                    minutes: next_minutes,
                    target: next_target.unwrap(),
                });
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
        assert_eq!(navigate_with_snacks(input), Some(54));
    }
}
