use num::abs;
use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::part1::{Pos, Step};

pub struct Grid {
    pub map: HashSet<Pos>,
    knots: [Pos; 10],
}

impl Grid {
    fn is_knot_adjacent(&self, knot: usize, other: usize) -> bool {
        let Pos(x, y) = self.knots[knot];
        let Pos(hx, hy) = self.knots[other];
        abs(x - hx) <= 1 && abs(y - hy) <= 1
    }

    fn print(&self) {
        let mut values = self
            .map
            .iter()
            .map(|Pos(x, y)| (*x, *y))
            .collect::<Vec<_>>();

        // values.push((self.head.0, self.head.1));

        let min_x = *values.iter().map(|(x, _)| x).min().unwrap() - 2;
        let max_x = *values.iter().map(|(x, _)| x).max().unwrap() + 2;
        let min_y = *values.iter().map(|(_, y)| y).min().unwrap() - 2;
        let max_y = *values.iter().map(|(_, y)| y).max().unwrap() + 2;

        for y in min_y..max_y + 1 {
            for x in min_x..max_x + 1 {
                match (x, y) {
                    (x, y) if self.map.contains(&Pos(x, y)) => print!("#"),
                    // (x, y) if self.head == Pos(x, y) => print!("H"),
                    // (x, y) if self.tail == Pos(x, y) => print!("T"),
                    _ => print!("."),
                }
            }
            println!()
        }
    }
}

fn simulate_tail(grid: &mut Grid) {
    for i in 1..grid.knots.len() {
        if grid.is_knot_adjacent(i - 1, i) {
            continue;
        }

        let Pos(x, y) = grid.knots[i];
        let Pos(hx, hy) = grid.knots[i - 1];

        let dx = (hx - x) / max((hx - x).abs(), 1);
        let dy = (hy - y) / max((hy - y).abs(), 1);
        // println!("dx: {}, dy: {}", dx, dy);

        grid.knots[i] = Pos(x + dx, y + dy);
    }

    grid.map.insert(grid.knots[9]);
}

fn run_step(grid: &mut Grid, step: &Step) {
    let (dx, dy) = step.heading;
    for _ in 0..step.dist {
        // println!("Step {}", i);
        grid.knots[0] = Pos(grid.knots[0].0 + dx, grid.knots[0].1 + dy);
        simulate_tail(grid);

        // grid.print();
    }
}

pub fn simulate_long_ropes(input: &str) -> usize {
    let mut grid = Grid {
        map: HashSet::new(),
        knots: [Pos(0, 0); 10],
    };
    grid.map.insert(*grid.knots.last().unwrap());

    input
        .lines()
        .map(Step::new)
        .for_each(|step| run_step(&mut grid, &step));

    grid.map.len()
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_movements() {
        let input = include_str!("../test.txt");
        assert_eq!(simulate_long_ropes(input), 1);
    }
    #[test]
    fn test_long_movements() {
        let input = include_str!("../test2.txt");
        assert_eq!(simulate_long_ropes(input), 36);
    }
}
