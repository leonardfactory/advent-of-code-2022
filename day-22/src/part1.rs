use std::{collections::HashMap, io::stdin};

use colored::Colorize;
use enum_iterator::{next_cycle, previous_cycle, Sequence};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new<T: Into<i32> + Copy>(x: T, y: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn forward(&self, direction: Direction) -> Self {
        use Direction::*;
        match direction {
            Up => Self::new(self.x, self.y - 1),
            Down => Self::new(self.x, self.y + 1),
            Left => Self::new(self.x - 1, self.y),
            Right => Self::new(self.x + 1, self.y),
        }
    }

    pub fn neighbors(&self) -> Vec<Pos> {
        vec![
            Pos::new(self.x - 1, self.y),
            Pos::new(self.x + 1, self.y),
            Pos::new(self.x, self.y - 1),
            Pos::new(self.x, self.y + 1),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PortalMap {
    pub vertical: Option<Pos>,
    pub vertical_dir: Option<Direction>,
    pub horizontal: Option<Pos>,
    pub horizontal_dir: Option<Direction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Open,
    Wall,
    Portal(PortalMap),
}

impl Tile {
    pub fn merge(&mut self, other: Self) {
        match (self, other) {
            (Self::Portal(p1), Self::Portal(p2)) => {
                *p1 = PortalMap {
                    vertical: p1.vertical.or(p2.vertical),
                    horizontal: p1.horizontal.or(p2.horizontal),
                    vertical_dir: p1.vertical_dir.or(p2.vertical_dir),
                    horizontal_dir: p1.horizontal_dir.or(p2.horizontal_dir),
                }
            }
            (t1, t2) => panic!("Can merge only portals, not {:?} and {:?}", t1, t2),
        }
    }

    pub fn is_portal(&self) -> bool {
        matches!(self, Self::Portal(_))
    }

    pub fn create_portal(tiles: &HashMap<Pos, Tile>, pos: Pos, neighbour: Pos) -> Self {
        // Vertical
        if pos.x - neighbour.x == 0 {
            let col_tiles = tiles
                .iter()
                .filter_map(|(p, t)| (p.x == pos.x && !t.is_portal()).then_some(p.y));

            Self::Portal(PortalMap {
                vertical: match neighbour.y - pos.y {
                    // South is empty
                    1 => Some(Pos::new(pos.x, col_tiles.min().unwrap())),
                    -1 => Some(Pos::new(pos.x, col_tiles.max().unwrap())),
                    _ => unreachable!(),
                },
                vertical_dir: None,
                horizontal: None,
                horizontal_dir: None,
            })
        } else {
            let row_tiles = tiles
                .iter()
                .filter_map(|(p, t)| (p.y == pos.y && !t.is_portal()).then_some(p.x));

            Self::Portal(PortalMap {
                vertical: None,
                vertical_dir: None,
                horizontal_dir: None,
                horizontal: match neighbour.x - pos.x {
                    // Right is empty
                    1 => Some(Pos::new(row_tiles.min().unwrap(), pos.y)),
                    -1 => Some(Pos::new(row_tiles.max().unwrap(), pos.y)),
                    _ => unreachable!(),
                },
            })
        }
    }
}

pub struct Board {
    pub tiles: HashMap<Pos, Tile>,
    pub min: Pos,
    pub max: Pos,
    pub square_size: i32,
}

impl Board {
    pub fn parse(input: &str) -> Self {
        let tiles: HashMap<_, _> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| match c {
                    '.' => Some((Pos::new(x as i32, y as i32), Tile::Open)),
                    '#' => Some((Pos::new(x as i32, y as i32), Tile::Wall)),
                    _ => None,
                })
            })
            .collect();

        let min = Pos {
            x: tiles.keys().map(|p| p.x).min().unwrap(),
            y: tiles.keys().map(|p| p.y).min().unwrap(),
        };

        let max = Pos {
            x: tiles.keys().map(|p| p.x).max().unwrap(),
            y: tiles.keys().map(|p| p.y).max().unwrap(),
        };

        Self {
            tiles,
            min,
            max,
            square_size: if max.x == 149 { 50 } else { 4 },
        }
    }

    pub fn start_position(&self) -> Player {
        let start_x = self
            .tiles
            .iter()
            .filter_map(|(pos, tile)| (tile == &Tile::Open && pos.y == 0).then_some(pos.x))
            .min()
            .unwrap();

        println!("Start: {}", start_x);

        Player {
            pos: Pos::new(start_x, 0),
            direction: Direction::Right,
        }
    }

    #[allow(dead_code)]
    pub fn print(&self, player: Option<Player>) {
        for y in self.min.y - 1..=self.max.y + 1 {
            for x in self.min.x - 1..=self.max.x + 1 {
                let pos = Pos::new(x, y);
                let tile = self.tiles.get(&pos).unwrap_or(&Tile::Empty);

                if player.map(|p| p.pos == pos).unwrap_or(false) {
                    print!("{}", player.unwrap().display());
                    continue;
                }

                print!(
                    "{}",
                    match tile {
                        Tile::Empty => " ".on_black(),
                        Tile::Open => ".".red().on_purple(),
                        Tile::Wall => "#".white().on_purple(),
                        Tile::Portal(_) => "P".cyan().on_black(),
                    }
                );
            }
            println!();
        }

        println!("\nPress to show next move..");
        let mut user_command = String::new();
        stdin().read_line(&mut user_command).ok();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player {
    pub pos: Pos,
    pub direction: Direction,
}

impl Player {
    pub fn password(&self) -> i32 {
        use Direction::*;
        let facing_score = match self.direction {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        };

        1000 * (self.pos.y + 1) + 4 * (self.pos.x + 1) + facing_score
    }

    fn display(&self) -> &'static str {
        match self.direction {
            Direction::Right => ">",
            Direction::Down => "v",
            Direction::Left => "<",
            Direction::Up => "^",
        }
    }
}

// Winding order -> clockwise
#[derive(Debug, Clone, Copy, PartialEq, Eq, Sequence)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Debug)]
pub enum Move {
    TurnLeft,
    TurnRight,
    Forward(i32),
}

impl Move {
    pub fn parse(input: &str) -> Vec<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+|R|L)").unwrap();
        }
        RE.find_iter(input)
            .map(|m| match m.as_str() {
                "R" => Self::TurnRight,
                "L" => Self::TurnLeft,
                n => Self::Forward(n.parse().unwrap()),
            })
            .collect()
    }
}

fn create_flat_board(input: &str) -> Board {
    let mut board = Board::parse(input);

    #[allow(clippy::needless_collect)]
    let edge_tiles: Vec<_> = board
        .tiles
        .iter()
        .flat_map(|(pos, _)| {
            pos.neighbors()
                .iter()
                .filter_map(|neighbour| {
                    board
                        .tiles
                        .get(neighbour)
                        .map_or(Some((*pos, *neighbour)), |_| None)
                })
                .collect::<Vec<_>>()
        })
        .collect();

    edge_tiles.into_iter().for_each(|(pos, neighbour)| {
        let portal = Tile::create_portal(&board.tiles, pos, neighbour);
        board
            .tiles
            .entry(neighbour)
            .or_insert(Tile::Portal(PortalMap {
                vertical: None,
                horizontal: None,
                vertical_dir: None,
                horizontal_dir: None,
            }))
            .merge(portal);
    });

    board
}

pub fn run_maze(input: &str) -> Player {
    let (board, moves) = input.split_once("\n\n").unwrap();
    let board = create_flat_board(board);
    let moves = Move::parse(moves);
    // board.print(None);

    let mut player = board.start_position();

    for m in moves {
        // println!("Move: {:?}", m);
        // board.print(Some(player));
        match m {
            Move::TurnLeft => player.direction = previous_cycle(&player.direction).unwrap(),
            Move::TurnRight => player.direction = next_cycle(&player.direction).unwrap(),
            Move::Forward(n) => {
                for _ in 0..n {
                    let forward_pos = board
                        .tiles
                        .get(&player.pos.forward(player.direction))
                        .unwrap();

                    player.pos = match forward_pos {
                        Tile::Open => player.pos.forward(player.direction),
                        Tile::Wall => player.pos,
                        Tile::Portal(portal) => {
                            let portaled_pos = match player.direction {
                                Direction::Left => portal.horizontal,
                                Direction::Right => portal.horizontal,
                                Direction::Up => portal.vertical,
                                Direction::Down => portal.vertical,
                            }
                            .unwrap();

                            match board.tiles.get(&portaled_pos).unwrap() {
                                Tile::Open => portaled_pos,
                                Tile::Wall => player.pos,
                                t => {
                                    unreachable!("Unexpected portal {:?} from {:?}", t, forward_pos)
                                }
                            }
                        }
                        Tile::Empty => unreachable!(),
                    }
                }
            }
        }
    }

    player
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(run_maze(input).password(), 6032);
    }
}
