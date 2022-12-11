use day_11::{part1::count_monkey_business, part2::count_unmanageable_monkey_business};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!(
        "Monkey business level: {}",
        count_monkey_business(input, 20)
    );
    println!(
        "Monkey unmanageable business level: {}",
        count_unmanageable_monkey_business(input, 10_000)
    );
    runner.end();
}
