use day_15::{part1::count_occupied, part2::distress_beacon};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Surely free: {}", count_occupied(input, 2_000_000));
    println!("Distress tuning: {}", distress_beacon(input, 4_000_000));
    runner.end();
}
