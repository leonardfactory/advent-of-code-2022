use day_21::{part1::parse_and_solve, part2::find_human_yell};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Expression result: {}", parse_and_solve(input));
    println!("Human value: {}", find_human_yell(input));
    runner.end();
}
