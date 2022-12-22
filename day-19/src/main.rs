use day_19::{part1::sum_quality_levels, part2::multiply_production};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    // This is incredibly slow, but it works for now.
    println!("Sum quality levels: {}", sum_quality_levels(input));
    println!("Multiply productions: {}", multiply_production(input));
    runner.end();
}
