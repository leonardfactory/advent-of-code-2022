use day_03::{part1::sum_rucksack_priorities, part2::sum_badges_priorities};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Rucksack priorities sum is: {}", sum_rucksack_priorities(input));
    println!("Rucksack badges sum is: {}", sum_badges_priorities(input));
    runner.end();
}
