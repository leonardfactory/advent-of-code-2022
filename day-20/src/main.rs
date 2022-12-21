use day_20::{part1::groove_coordinates, part2::decrypt_coordinates};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Groove Coordinates: {}", groove_coordinates(input));
    println!("Decrypted Coordinates: {}", decrypt_coordinates(input));
    runner.end();
}
