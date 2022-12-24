use day_23::{part1::simulate_elves, part2::find_stop_round};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Empty grid count: {}", simulate_elves(input));
    println!("Rounds to stop: {}", find_stop_round(input));
    runner.end();
}
