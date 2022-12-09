use day_08::{
    part1::{count_visible_trees, parse_grid},
    part2::max_scenic_score,
};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Number of visible trees: {}", count_visible_trees(input));

    let grid = parse_grid(input);
    println!("Max scenic score: {}", max_scenic_score(&grid));
    runner.end();
}
