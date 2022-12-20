use core::{fmt::Debug, panic};
use std::{
    cmp::{max, min},
    collections::HashMap,
    fmt::{self, Formatter},
};

use colored::Colorize;
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Chamber {
    pub tiles: HashMap<(i64, i64), Tile>,
    pub max_heights: Vec<i64>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Tile {
    Empty,
    Rock,
    Moving,
}

impl Chamber {
    pub fn empty() -> Self {
        let mut tiles = HashMap::new();
        (0..7).for_each(|x| {
            tiles.insert((x, -1), Tile::Rock);
        });

        Self {
            tiles,
            max_heights: vec![0; 7],
        }
    }

    pub fn add(&mut self, rock: &Rock, x: i64, y: i64) {
        rock.tiles.iter().for_each(|((dx, dy), tile)| {
            let (nx, ny) = (x + dx, y + dy);
            if tile != &Tile::Empty {
                self.tiles.insert((nx, ny), *tile);
            }
        });
    }

    pub fn are_equal(&self, y1: i64, y2: i64) -> Option<i64> {
        let mut stopped = vec![false; 7];
        let mut max_y = 0;
        for y in 0..50 {
            max_y += 1;
            if stopped.iter().all(|&s| s) {
                break;
            }
            for x in 0..7 {
                // if stopped[x as usize] {
                //     continue;
                // }
                if self.tiles.get(&(x, y1 - y - 1)) != self.tiles.get(&(x, y2 - y - 1)) {
                    return None;
                }
                if let Some(Tile::Rock) = self.tiles.get(&(x, y1 - y - 1)) {
                    stopped[x as usize] = true;
                }
            }
        }
        if stopped.iter().any(|&s| !s) {
            println!("Not sufficient height");
            self.print_at_y(y1);
            self.print_at_y(y2);
            return None;
        }
        Some(max_y as i64)
    }

    pub fn overlaps(&mut self, rock: &Rock, x: i64, y: i64) -> bool {
        rock.tiles.iter().any(|((dx, dy), tile)| {
            let (nx, ny) = (x + dx, y + dy);
            match tile {
                Tile::Empty => false,
                Tile::Rock => self.tiles.get(&(nx, ny)) == Some(&Tile::Rock),
                Tile::Moving => panic!("Unexpected moving tile in rock!"),
            }
        })
    }

    #[allow(dead_code)]
    pub fn print(&self, other: Option<(&Rock, i64, i64)>) {
        let mut copy = self.clone();
        if let Some((rock, x, y)) = other {
            let mut cloned_rock = rock.clone();
            cloned_rock.tiles.iter_mut().for_each(|(_, tile)| {
                if tile == &Tile::Rock {
                    *tile = Tile::Moving;
                }
            });
            copy.add(&cloned_rock, x, y);
        }
        // clearscreen::clear().ok();
        let max_height = copy.tiles.iter().map(|((_, y), _)| *y).max().unwrap_or(0) + 1;
        println!(
            "\nChamber (max height = {}): (rx={},ry={})",
            max_height,
            other.map(|r| r.1).unwrap_or(0),
            other.map(|r| r.2).unwrap_or(0)
        );
        for y in ((max_height - 30).max(0)..max_height).rev() {
            for x in 0..7 {
                let tile = copy.tiles.get(&(x, y)).unwrap_or(&Tile::Empty);
                print!(
                    "{}",
                    match tile {
                        Tile::Empty => ".".cyan(),
                        Tile::Rock => "#".white(),
                        Tile::Moving => "@".red(),
                    }
                );
            }
            println!();
        }

        // let mut c = String::new();
        // io::stdin().read_line(&mut c).ok();
    }

    pub fn print_at_y(&self, y: i64) {
        println!("Chamber at y = {}:", y);
        for y in ((y - 30).max(0)..y).rev() {
            for x in 0..7 {
                let tile = self.tiles.get(&(x, y)).unwrap_or(&Tile::Empty);
                print!(
                    "{}",
                    match tile {
                        Tile::Empty => ".".cyan(),
                        Tile::Rock => "#".white(),
                        Tile::Moving => "@".red(),
                    }
                );
            }
            println!();
        }

        // let mut c = String::new();
        // io::stdin().read_line(&mut c).ok();
    }
}

#[derive(Clone)]
pub struct Rock {
    pub tiles: HashMap<(i64, i64), Tile>,
    pub shape: String,
    pub height: i64,
    pub width: i64,
}

impl Debug for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\n{}", self.shape)
    }
}

impl Rock {
    pub fn parse(shape: &str) -> Self {
        let mut tiles = HashMap::new();
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        for c in shape.lines().rev().join("\n").chars() {
            match c {
                '#' => {
                    tiles.insert((x, y), Tile::Rock);
                    width = width.max(x + 1);
                    x += 1;
                }
                '.' => {
                    tiles.insert((x, y), Tile::Empty);
                    width = width.max(x + 1);
                    x += 1;
                }
                '\n' => {
                    x = 0;
                    y += 1;
                }
                _ => panic!("Unknown character: {}", c),
            }
        }
        Self {
            tiles,
            shape: shape.to_string(),
            width,
            height: y + 1,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Flow {
    Left,
    Right,
}

pub fn parse_flow(flow: &str) -> Vec<Flow> {
    flow.chars()
        .map(|f| match f {
            '<' => Flow::Left,
            '>' => Flow::Right,
            _ => panic!("Unknown character: {}", f),
        })
        .collect_vec()
}

pub fn get_rocks() -> Vec<Rock> {
    vec![
        Rock::parse("####"),
        Rock::parse(".#.\n###\n.#."),
        Rock::parse("..#\n..#\n###"),
        Rock::parse("#\n#\n#\n#"),
        Rock::parse("##\n##"),
    ]
}

pub fn compute_height(input: &str, count: i64) -> i64 {
    let rocks = get_rocks();
    let flow = parse_flow(input);
    println!("Flow len: {:?}", flow.len());
    let mut flow_index = 0;

    let mut chamber = Chamber::empty();
    // let mut spawn_y = 3;
    let mut max_height = 0_i64;

    for i in 0..count {
        let rock = &rocks[(i % rocks.len() as i64) as usize];
        let mut rock_x = 2;
        let mut rock_y = max_height + 3;

        let mut j = 0;
        loop {
            // chamber.print(Some((rock, rock_x, rock_y)));
            match j % 2 {
                // Move X
                0 => {
                    let delta = flow[flow_index % flow.len()];
                    flow_index += 1;
                    let next_x = match delta {
                        Flow::Left => max(0, rock_x - 1),
                        Flow::Right => min(rock_x + 1, 7 - rock.width),
                    };

                    if chamber.overlaps(rock, next_x, rock_y) {
                        j += 1;
                        continue;
                    } else {
                        rock_x = next_x;
                    }
                }
                // Move Y
                1 => {
                    if chamber.overlaps(rock, rock_x, rock_y - 1) {
                        break;
                    }
                    rock_y -= 1;
                }
                _ => panic!("Unknown j: {}", j),
            }
            j += 1;
        }

        // println!("Adding rock {:?} at ({}, {})", rock, rock_x, rock_y);
        max_height = max(max_height, rock_y + rock.height);
        chamber.add(rock, rock_x, rock_y);
        // chamber.print(None);
        // clearscreen::clear().ok();
        if i % 1_000_000 == 0 {
            println!("Max height: {}, i={}", max_height, i);
        }
    }

    // chamber.print(None);
    max_height
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(compute_height(input, 2022), 3068);
    }
}
