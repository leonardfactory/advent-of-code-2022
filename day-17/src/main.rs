use day_17::{part1::compute_height, part2::compute_height_simulate};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Max tetris height: {}", compute_height(input, 2022));
    println!(
        "Max height simulate: {}",
        compute_height_simulate(input, 1_000_000_000_000_i64)
    );
    runner.end();
}
