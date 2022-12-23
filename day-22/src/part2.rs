use enum_iterator::{next_cycle, previous_cycle};
use std::collections::HashMap;

use crate::part1::{
    Board,
    Direction::{self, *},
    Move, Player, PortalMap, Pos, Tile,
};

fn direction_to_range(size: i32, dir: Direction) -> Vec<Pos> {
    match dir {
        Up => (0..size).map(|x| Pos::new(x, -1)).collect(),
        Left => (0..size).rev().map(|y| Pos::new(-1, y)).collect(),
        Right => (0..size).map(|y| Pos::new(size, y)).collect(),
        Down => (0..size).rev().map(|x| Pos::new(x, size)).collect(),
    }
}

impl Pos {
    fn in_quadrant(&self, coords: &Pos, size: i32) -> Pos {
        Self {
            x: self.x + coords.x * size,
            y: self.y + coords.y * size,
        }
    }
}

fn create_cube_board(input: &str) -> Board {
    let mut board = Board::parse(input);
    let size = board.square_size;

    // let coords: Vec<_> = (0..(board.max.x + 1) / size)
    //     .cartesian_product(0..(board.max.y + 1) / size)
    //     .collect();

    // Hardcoded map of `input.txt`
    let id_to_coords: HashMap<_, _> = [
        (1, Pos::new(0, 3)),
        (2, Pos::new(0, 2)),
        (3, Pos::new(1, 2)),
        (4, Pos::new(1, 1)),
        (5, Pos::new(1, 0)),
        (6, Pos::new(2, 0)),
    ]
    .into_iter()
    .collect();

    let mappings = vec![
        ((1, Left), (5, Up)),
        ((1, Down), (6, Up)),
        ((1, Right), (3, Down)),
        ((2, Left), (5, Left)),
        ((2, Up), (4, Left)),
        ((3, Right), (6, Right)),
        ((4, Right), (6, Down)),
    ];

    mappings
        .into_iter()
        .for_each(|((from, from_dir), (to, to_dir))| {
            let coord_from = id_to_coords.get(&from).unwrap();
            let coord_to = id_to_coords.get(&to).unwrap();

            for (from_pos, to_pos) in direction_to_range(size, from_dir)
                .iter()
                .zip(direction_to_range(size, to_dir).iter().rev())
            {
                let from_pos = from_pos.in_quadrant(coord_from, size);
                let to_pos = to_pos.in_quadrant(coord_to, size);

                create_cube_portal(&mut board, from_pos, from_dir, to_pos, to_dir);
                create_cube_portal(&mut board, to_pos, to_dir, from_pos, from_dir);
            }
        });

    board
}

impl Direction {
    fn rotate(&self, rotation: i32) -> Direction {
        let mut final_dir = *self;
        for _ in 0..rotation {
            final_dir = next_cycle(&final_dir).unwrap();
        }
        final_dir
    }
}

fn create_cube_portal(
    board: &mut Board,
    from_pos: Pos,
    from_dir: Direction,
    to_pos: Pos,
    to_dir: Direction,
) {
    let target_dir = to_dir.rotate(2); // Reverse
    let portal_map = match from_dir {
        Left | Right => PortalMap {
            vertical: None,
            vertical_dir: None,
            horizontal: Some(to_pos),
            horizontal_dir: Some(target_dir),
        },
        Up | Down => PortalMap {
            vertical: Some(to_pos),
            vertical_dir: Some(target_dir),
            horizontal: None,
            horizontal_dir: None,
        },
    };

    board
        .tiles
        .entry(from_pos)
        .or_insert(Tile::Portal(PortalMap {
            vertical: None,
            horizontal: None,
            vertical_dir: None,
            horizontal_dir: None,
        }))
        .merge(Tile::Portal(portal_map));
}

/**
 * The algorithm is hardcoded for input.txt cube structure and does not
 * work on the example input.
 */
pub fn run_cube_maze(input: &str) -> Player {
    let (board, moves) = input.split_once("\n\n").unwrap();
    let board = create_cube_board(board);
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
                            let (portaled_pos, portaled_dir) = match player.direction {
                                Left => (portal.horizontal, portal.horizontal_dir),
                                Right => (portal.horizontal, portal.horizontal_dir),
                                Up => (portal.vertical, portal.vertical_dir),
                                Down => (portal.vertical, portal.vertical_dir),
                            };

                            let portaled_dir = portaled_dir.unwrap();
                            let portaled_pos = portaled_pos.unwrap().forward(portaled_dir);
                            // board.print(Some(player));
                            // println!(
                            //     "Portal {:?} (dir {:?}) -> {:?} (dir {:?}",
                            //     player.pos, player.direction, portaled_pos, portaled_dir
                            // );

                            match board.tiles.get(&portaled_pos).unwrap() {
                                Tile::Open => {
                                    // Side effect for direction
                                    player.direction = portaled_dir;
                                    portaled_pos
                                }
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
