use day_09::{part1::simulate_ropes, part2::simulate_long_ropes};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Visited cells: {}", simulate_ropes(input));
    println!(
        "Visited cells with long ropes: {}",
        simulate_long_ropes(input)
    );
    runner.end();
}
