use day_16::{part1::max_pressure, part2::max_pressure_in_two};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Max pressure: {}", max_pressure(input));
    println!("Max pressure in two: {}", max_pressure_in_two(input));
    runner.end();
}
