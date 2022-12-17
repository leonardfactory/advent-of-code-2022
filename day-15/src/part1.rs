use std::cmp::{max, min};
use std::fmt::{Debug, Formatter};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn dist(&self, other: &Pos) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x:{}, y:{})", self.x, self.y)
    }
}

pub struct Scanner {
    pub center: Pos,
    pub beacon: Pos,
    pub radius: i32,
}

impl Scanner {
    pub fn new(center: Pos, beacon: Pos) -> Self {
        Self {
            center,
            beacon,
            radius: center.dist(&beacon),
        }
    }

    pub fn parse(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
            )
            .unwrap();
        }

        let captures = RE.captures(input).unwrap();
        let center = Pos {
            x: captures.get(1).unwrap().as_str().parse().unwrap(),
            y: captures.get(2).unwrap().as_str().parse().unwrap(),
        };
        let beacon = Pos {
            x: captures.get(3).unwrap().as_str().parse().unwrap(),
            y: captures.get(4).unwrap().as_str().parse().unwrap(),
        };

        Self::new(center, beacon)
    }

    pub fn range_at_y(&self, y: i32) -> Option<Range> {
        let delta = self.center.y - y;
        if delta.abs() > self.radius {
            return None;
        }

        // let occupied = (self.radius * 2) - delta.abs() * 2 + 1;
        let start_x = self.center.x - self.radius + delta.abs();
        let end_x = self.center.x + self.radius - delta.abs();
        Some(Range::new(start_x, end_x))
    }
}

// Inclusive range
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: i32,
    pub end: i32,
}

impl Range {
    pub fn new(start: i32, end: i32) -> Self {
        if start > end {
            panic!("Unexpected range: {} - {}", start, end);
        }
        Self { start, end }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> i32 {
        (self.end - self.start).abs() + 1
    }

    pub fn intersect(&self, other: &Range) -> Option<Self> {
        if self.start > other.end || self.end < other.start {
            return None;
        }

        Some(Self {
            start: max(self.start, other.start),
            end: min(self.end, other.end),
        })
    }

    pub fn contains(&self, pos: &Pos) -> bool {
        self.start <= pos.x && self.end >= pos.x
    }
}

impl Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

pub fn count_ranges(ranges: &[Range]) -> i32 {
    ranges.iter().map(|r| r.len()).sum()
}

pub fn ranges_at_row(scanners: &[Scanner], row: i32) -> (Vec<Range>, Vec<Range>) {
    let (ranges, subtracts) = scanners
        .iter()
        .fold((vec![], vec![]), |(ranges, subtracts), s| {
            // println!("BEACON: {:?} {:?} r={}", s.center, s.beacon, s.radius);
            match s.range_at_y(row) {
                Some(range) => {
                    // println!(" Range: {:?}", range);
                    let mut next_ranges: Vec<Range> = vec![];
                    let mut next_subtracts: Vec<Range> = vec![];

                    next_ranges.push(range);

                    for other in ranges.iter() {
                        if let Some(overlap) = range.intersect(other) {
                            // println!(" - overlap: {:?} (with {:?})", overlap, other);
                            next_subtracts.push(overlap);
                        }
                    }

                    for other in subtracts.iter() {
                        if let Some(overlap) = range.intersect(other) {
                            // println!(" + overlap: {:?} (with {:?})", overlap, other);
                            next_ranges.push(overlap);
                        }
                    }

                    next_ranges.extend(ranges);
                    next_subtracts.extend(subtracts);

                    // let plus = count_ranges(&next_ranges);
                    // let minus = count_ranges(&next_subtracts);
                    // println!("[total={}, +{}, -{}]", plus - minus, plus, minus);

                    (next_ranges, next_subtracts)
                }
                None => (ranges, subtracts),
            }
        });

    (ranges, subtracts)
}

pub fn count_occupied(input: &str, row: i32) -> i32 {
    let scanners = input.lines().map(Scanner::parse).collect_vec();
    let (ranges, subtracts) = ranges_at_row(&scanners, row);

    let beacons_diff = scanners
        .iter()
        .unique_by(|s| s.beacon)
        .filter(|s| s.beacon.y == row)
        .count() as i32;

    let ranges_sum = count_ranges(&ranges);
    let subtracts_sum = count_ranges(&subtracts);

    ranges_sum - subtracts_sum - beacons_diff
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_parsing() {
        let scanner = Scanner::parse("Sensor at x=10, y=-5: closest beacon is at x=5, y=-10");
        assert_eq!(scanner.center, Pos { x: 10, y: -5 });
        assert_eq!(scanner.beacon, Pos { x: 5, y: -10 });
        assert_eq!(scanner.radius, 10);
    }

    #[test]
    fn test_distance() {
        let scanner = Scanner::parse("Sensor at x=8, y=7: closest beacon is at x=2, y=10");
        assert_eq!(scanner.radius, 9);
        assert_eq!(scanner.range_at_y(16).unwrap().len(), 1);
        assert_eq!(scanner.range_at_y(-1).unwrap().len(), 3);
        assert_eq!(scanner.range_at_y(17), None);
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(count_occupied(input, 10), 26);
    }
}
