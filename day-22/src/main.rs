use day_22::{part1::run_maze, part2::run_cube_maze};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Maze password is: {}", run_maze(input).password());
    println!("Maze 3d password is: {}", run_cube_maze(input).password());
    runner.end();
}
