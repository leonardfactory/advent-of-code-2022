use day_13::{part1::count_ordered_pairs, part2::decoder_packet_indices};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!("Summed pairs indices: {}", count_ordered_pairs(input));
    println!("Decoder key: {}", decoder_packet_indices(input));
    runner.end();
}
