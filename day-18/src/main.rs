use day_18::{part1::exposed_surface, part2::exposed_external_surface};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Exposed sides: {}", exposed_surface(input));
    println!(
        "Exposed external sides: {}",
        exposed_external_surface(input)
    );
    runner.end();
}
