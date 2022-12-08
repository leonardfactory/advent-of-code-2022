use day_06::{part1::marker_position, part2::message_position};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Marker position is: {}", marker_position(input));
    println!("Message position is: {}", message_position(input));
    runner.end();
}
