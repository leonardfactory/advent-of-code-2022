use colored::Colorize;
use grid::Grid;

pub fn parse_grid(data: &str) -> Grid<usize> {
    let lines: Vec<Vec<usize>> = data
        .lines()
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let mut grid = Grid::new(0, lines[0].len());
    lines.iter().for_each(|l| grid.push_row(l.to_vec()));
    grid
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

fn grid_get_at(grid: &Grid<usize>, pos: Pos) -> usize {
    *grid.get(pos.y, pos.x).unwrap()
}

pub fn is_invisible(pos: Pos, grid: &Grid<usize>) -> bool {
    let height = grid_get_at(grid, pos);
    (0..pos.x).any(|x| *grid.get(pos.y, x).unwrap() >= height)
        && (pos.x + 1..grid.cols()).any(|x| *grid.get(pos.y, x).unwrap() >= height)
        && (0..pos.y).any(|y| *grid.get(y, pos.x).unwrap() >= height)
        && (pos.y + 1..grid.rows()).any(|y| *grid.get(y, pos.x).unwrap() >= height)
}

pub fn count_visible_trees(data: &str) -> usize {
    let grid = parse_grid(data);
    let mut count = (grid.cols() + grid.rows() - 2) * 2;
    println!("rows {} cols {}", grid.rows(), grid.cols());
    for y in 1..grid.rows() - 1 {
        for x in 1..grid.cols() - 1 {
            if !is_invisible(Pos { x, y }, &grid) {
                // println!("----- {} (={}) visible at {},{}", grid_get_at(&grid, Pos { x, y }), *grid.get(y, x).unwrap(),  x+1, y+1);
                // print_grid_highlight(&grid, Pos { x, y });
                count += 1;
            }
        }
    }
    count
}

#[allow(dead_code)]
fn print_grid_highlight(grid: &Grid<usize>, pos: Pos) {
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            let height = grid_get_at(grid, Pos { x, y });
            if x == pos.x && y == pos.y {
                print!("{}", height.to_string().red().bold());
            } else {
                print!("{}", height);
            }
        }
        println!();
    }
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(count_visible_trees(input), 21);
    }
}
