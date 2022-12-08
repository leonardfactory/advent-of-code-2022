use day_02::{
    part1::{parse, winning_score},
    part2::transform_strategy,
};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = parse(include_str!("../input.txt"));
    println!("Winning score is: {}", winning_score(&input));
    let transformed: Vec<_> = input.iter().map(transform_strategy).collect();
    println!("Definitive score is: {}", winning_score(&transformed));
    runner.end();
}
