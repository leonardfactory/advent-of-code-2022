use std::{cmp::max, collections::HashSet};

use num::abs;

pub fn part1() {}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos(pub i32, pub i32);

pub struct Grid {
    pub map: HashSet<Pos>,
    head: Pos,
    tail: Pos,
}

impl Grid {
    fn is_tail_adjacent(&self) -> bool {
        let Pos(x, y) = self.tail;
        let Pos(hx, hy) = self.head;
        abs(x - hx) <= 1 && abs(y - hy) <= 1
    }

    #[allow(dead_code)]
    fn print(&self) {
        let mut values = self
            .map
            .iter()
            .map(|Pos(x, y)| (*x, *y))
            .collect::<Vec<_>>();

        values.push((self.head.0, self.head.1));

        let min_x = *values.iter().map(|(x, _)| x).min().unwrap() - 2;
        let max_x = *values.iter().map(|(x, _)| x).max().unwrap() + 2;
        let min_y = *values.iter().map(|(_, y)| y).min().unwrap() - 2;
        let max_y = *values.iter().map(|(_, y)| y).max().unwrap() + 2;

        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                match (x, y) {
                    (x, y) if self.head == Pos(x, y) => print!("H"),
                    (x, y) if self.tail == Pos(x, y) => print!("T"),
                    _ => print!("."),
                }
            }
            println!()
        }
    }
}

pub struct Step {
    pub heading: (i32, i32),
    pub dist: usize,
}

impl Step {
    pub fn new(data: &str) -> Self {
        let (dir, dist) = data.split_once(' ').unwrap();
        let dist = dist.parse::<usize>().unwrap();
        let heading = match dir {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("Invalid direction"),
        };
        Self { heading, dist }
    }
}

fn simulate_tail(grid: &mut Grid) {
    if grid.is_tail_adjacent() {
        return;
    }

    let Pos(x, y) = grid.tail;
    let Pos(hx, hy) = grid.head;

    let dx = (hx - x) / max((hx - x).abs(), 1);
    let dy = (hy - y) / max((hy - y).abs(), 1);
    // println!("dx: {}, dy: {}", dx, dy);

    grid.tail = Pos(x + dx, y + dy);
    grid.map.insert(grid.tail);
}

fn run_step(grid: &mut Grid, step: &Step) {
    let (dx, dy) = step.heading;
    for _i in 0..step.dist {
        // println!("Step {}", i);
        grid.head = Pos(grid.head.0 + dx, grid.head.1 + dy);
        simulate_tail(grid);

        // grid.print();
    }
}

pub fn simulate_ropes(input: &str) -> usize {
    let mut grid = Grid {
        map: HashSet::new(),
        head: Pos(0, 0),
        tail: Pos(0, 0),
    };
    grid.map.insert(grid.tail);

    input
        .lines()
        .map(Step::new)
        .for_each(|step| run_step(&mut grid, &step));

    grid.map.len()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_movements() {
        let input = include_str!("../test.txt");
        assert_eq!(simulate_ropes(input), 13);
    }
}
