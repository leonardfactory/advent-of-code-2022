pub mod bounds;
pub mod pos;

pub use bounds::*;
pub use pos::*;

use std::{collections::HashMap, fmt::Display, ops::Index};

pub type Neighbor<'a, T> = (Pos, &'a T);

/// A trait for types that can be displayed as a tile on a map.
pub trait TileDisplay {
    fn map_print(&self) -> Box<dyn Display>;
}

/// A generic map, usually parsed from an input file, composed of tiles.
/// Each tile is stored in a HashMap, indexed by its position, in order
/// to allow for fast lookup and virtually unlimited size.
#[derive(Clone, Debug)]
pub struct Map<T> {
    pub tiles: HashMap<Pos, T>,
    pub bounds: Bounds,
}

impl<T> Map<T> {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::new(),
            bounds: Bounds::empty(),
        }
    }

    pub fn update_bounds(&mut self) {
        self.bounds.max.x = self.tiles.keys().map(|p| p.x).max().unwrap_or(0);
        self.bounds.max.y = self.tiles.keys().map(|p| p.y).max().unwrap_or(0);
        self.bounds.min.x = self.tiles.keys().map(|p| p.x).min().unwrap_or(0);
        self.bounds.min.y = self.tiles.keys().map(|p| p.y).min().unwrap_or(0);
    }

    pub fn parse(input: &str, mut parse_tile: impl FnMut(char, usize, usize) -> Option<T>) -> Self {
        let mut map = Self::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if let Some(tile) = parse_tile(c, x, y) {
                    map.tiles.insert(Pos::new(x, y), tile);
                }
            }
        }

        map.update_bounds();

        map
    }

    pub fn get(&self, pos: Pos) -> Option<&T> {
        self.tiles.get(&pos)
    }

    pub fn all_neighbors(&self, pos: Pos) -> Vec<Neighbor<T>> {
        pos.all_neighbors()
            .into_iter()
            .filter_map(|p| self.get(p).map(|t| (p, t)))
            .collect()
    }
}

impl<T: TileDisplay> Map<T> {
    pub fn print(&self) {
        println!("\nMap:");
        for y in self.bounds.min.y..=self.bounds.max.y {
            for x in self.bounds.min.x..=self.bounds.max.x {
                let pos = Pos::new(x, y);
                if let Some(tile) = self.get(pos) {
                    print!("{}", tile.map_print());
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

impl<T> Default for Map<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Index<Pos> for Map<T> {
    type Output = T;

    fn index(&self, pos: Pos) -> &T {
        &self.tiles[&pos]
    }
}
