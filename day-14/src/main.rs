use day_14::{part1::count_sands_before_rest, part2::count_sands_with_floor};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Number of particles: {}", count_sands_before_rest(input));
    println!("Number with floor: {}", count_sands_with_floor(input));
    runner.end();
}
