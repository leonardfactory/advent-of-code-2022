use day_05::{part1::calculate_top, part2::calculate_top_9001};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Top word on crates is: {}", calculate_top(input));
    println!(
        "Top word on crates with 9001 is: {}",
        calculate_top_9001(input)
    );
    runner.end();
}
