use grid::Grid;

#[derive(Debug, Copy, Clone)]
pub struct Pos {
    x: isize,
    y: isize,
}

#[derive(Debug, Copy, Clone)]
pub struct Direction {
    dx: isize,
    dy: isize,
}

impl Pos {
    pub fn move_by(&self, dir: &Direction) -> Self {
        Self {
            x: self.x + dir.dx,
            y: self.y + dir.dy,
        }
    }

    pub fn from_grid(&self, grid: &Grid<usize>) -> Option<usize> {
        if self.x < 0
            || self.x >= grid.cols() as isize
            || self.y < 0
            || self.y >= grid.rows() as isize
        {
            None
        } else {
            Some(*grid.get(self.y as usize, self.x as usize).unwrap())
        }
    }
}

pub fn count_trees_in_direction(grid: &Grid<usize>, pos: Pos, dir: Direction) -> usize {
    let height = pos.from_grid(grid).unwrap();
    let mut tree = pos.move_by(&dir);
    let mut count = 0;

    while let Some(h) = tree.from_grid(grid) {
        count += 1;

        if h >= height {
            break;
        }

        tree.x += dir.dx;
        tree.y += dir.dy;
    }
    count
}

const DIRS: &[Direction] = &[
    Direction { dx: 0, dy: -1 },
    Direction { dx: 0, dy: 1 },
    Direction { dx: -1, dy: 0 },
    Direction { dx: 1, dy: 0 },
];

pub fn tree_scenic_score(grid: &Grid<usize>, pos: Pos) -> usize {
    let score = DIRS
        .iter()
        .map(|dir| count_trees_in_direction(grid, pos, *dir))
        .product();
    score
}

pub fn max_scenic_score(grid: &Grid<usize>) -> usize {
    let mut max = 0;
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let score = tree_scenic_score(
                grid,
                Pos {
                    x: x as isize,
                    y: y as isize,
                },
            );
            if score > max {
                max = score;
            }
        }
    }
    max
}

#[cfg(test)]
pub mod tests {
    use crate::{part1::parse_grid, part2::*};

    #[test]
    fn test_example_count() {
        let input = include_str!("../test.txt");
        let grid = parse_grid(input);
        assert_eq!(
            count_trees_in_direction(&grid, Pos { x: 2, y: 1 }, Direction { dx: 0, dy: -1 }),
            1
        );
        assert_eq!(
            count_trees_in_direction(&grid, Pos { x: 2, y: 1 }, Direction { dx: 0, dy: 1 }),
            2
        );
        assert_eq!(tree_scenic_score(&grid, Pos { x: 2, y: 1 }), 4);
        assert_eq!(tree_scenic_score(&grid, Pos { x: 2, y: 3 }), 8);
    }
}
