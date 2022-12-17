use crate::part1::{count_ranges, ranges_at_row, Pos, Range, Scanner};
use itertools::Itertools;

fn clamp_ranges(ranges: &mut Vec<Range>, max: i32) {
    for r in ranges.iter_mut() {
        r.start = r.start.max(0);
        r.end = r.end.min(max);
    }
    ranges.retain(|r| r.start >= 0 && r.end <= max);
}

pub fn occupied_at_row_ranged(
    scanners: &[Scanner],
    row: i32,
    coordinate_space: i32,
) -> Option<Pos> {
    let (mut ranges, mut subtracts) = ranges_at_row(scanners, row);
    clamp_ranges(&mut ranges, coordinate_space);
    clamp_ranges(&mut subtracts, coordinate_space);

    let occupied_positions = count_ranges(&ranges) - count_ranges(&subtracts);
    if occupied_positions == coordinate_space + 1 {
        return None;
    }

    let x = ranges
        .iter()
        .filter(|&r| {
            ranges.iter().all(|r2| {
                !r2.contains(&Pos {
                    x: r.end + 1,
                    y: row,
                })
            })
        })
        .map(|r| r.end + 1)
        .min()
        .unwrap();

    Some(Pos { x, y: row })
}

pub fn distress_beacon(input: &str, coordinate_space: i32) -> i64 {
    let scanners = input.lines().map(Scanner::parse).collect_vec();

    for j in 0..=coordinate_space {
        if let Some(pos) = occupied_at_row_ranged(&scanners, j, coordinate_space) {
            println!("Found at {:?}", pos);
            return (pos.x as i64) * 4_000_000 + pos.y as i64;
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(distress_beacon(input, 20), 56000011);
    }
}
