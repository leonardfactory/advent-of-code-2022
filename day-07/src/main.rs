use day_07::{part1::find_directories_by_total_size, part2::find_deletable_directory_size};
use runner::Runner;

fn main() {
    let runner = Runner::start();
    let input = include_str!("../input.txt");
    println!(
        "Directories < 100k: {}",
        find_directories_by_total_size(input)
    );
    println!(
        "Size of directory to be deleted: {}",
        find_deletable_directory_size(input)
    );
    runner.end();
}
