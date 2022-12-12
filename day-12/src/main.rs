use day_12::{part1::shortest_path, part2::shortest_path_from_anywhere};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Shortest path: {}", shortest_path(input).unwrap());
    println!(
        "Shortest path from 'a': {}",
        shortest_path_from_anywhere(input).unwrap()
    );
    runner.end();
}
