use day_24::{part1::navigate_blizzards, part2::navigate_with_snacks};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Min moves: {}", navigate_blizzards(input).unwrap());
    println!("Back to snack: {}", navigate_with_snacks(input).unwrap());
    runner.end();
}
