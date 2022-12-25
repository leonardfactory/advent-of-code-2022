use day_25::part1::count_fuel_snafu;
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Total SNAFU fuel: {}", count_fuel_snafu(input));
    runner.end();
}
