use day_10::{part1::signal_strength, part2::render_crt};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Signal strength: {}", signal_strength(input));
    println!("CRT Shows:");
    render_crt(input);
    runner.end();
}
