use day_01::{
    part1::{max_elf, order_elves},
    part2::top_three_elves,
};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    println!("Hello, advent of code 2022!");
    let input = order_elves(include_str!("../input.txt"));
    println!("Max is: {}", max_elf(&input));
    println!("Top 3 is: {}", top_three_elves(&input));
    runner.end();
}
