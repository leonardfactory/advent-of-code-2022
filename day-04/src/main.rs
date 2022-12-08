use day_04::{
    part1::{has_overlaps, parse_line},
    part2::has_partial_overlap,
};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!(
        "Overlaps count is: {}",
        input.lines().map(parse_line).filter(has_overlaps).count()
    );
    println!(
        "Partial overlaps count is: {}",
        input
            .lines()
            .map(parse_line)
            .filter(has_partial_overlap)
            .count()
    );
    runner.end();
}
