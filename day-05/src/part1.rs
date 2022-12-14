use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

fn reorder(stacks: &mut [Vec<char>], rules: &[Move]) {
    for rule in rules {
        let stack_from_len = stacks[rule.from].len();
        let mut to_move = stacks[rule.from]
            .drain((stack_from_len - rule.amount)..)
            .rev()
            .collect_vec();
        stacks[rule.to].append(&mut to_move);
    }
}

pub fn calculate_top(data: &str) -> String {
    let (mut stacks, rules) = parse(data);
    reorder(&mut stacks, &rules);
    let top = stacks.iter().map(|s| s.last().unwrap_or(&' ')).join("");
    top
}

pub fn parse(data: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let (raw_stacks, raw_rules) = data.split_once("\n\n").unwrap();

    let stacks = parse_stacks(raw_stacks);
    let rules = parse_rules(raw_rules);
    (stacks, rules)
}

fn parse_stacks(data: &str) -> Vec<Vec<char>> {
    let matrix = data
        .lines()
        .map(|l| l.chars().skip(1).step_by(4).collect::<Vec<_>>())
        .collect_vec();
    let stacks_count = matrix[0].len();
    let mut stacks: Vec<Vec<char>> = vec![vec![]; stacks_count];
    for row in matrix.iter().rev().skip(1) {
        for (i, c) in row.iter().enumerate() {
            if ' ' != *c {
                stacks[i].push(*c);
            }
        }
    }
    stacks
}

#[derive(Debug)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub amount: usize,
}

fn parse_rules(data: &str) -> Vec<Move> {
    lazy_static! {
        static ref MOVE_RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }

    data.lines()
        .map(|line| {
            let captures = MOVE_RE.captures(line).unwrap();
            Move {
                amount: captures[1].parse().unwrap(),
                from: captures[2].parse::<usize>().unwrap() - 1,
                to: captures[3].parse::<usize>().unwrap() - 1,
            }
        })
        .collect()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let top = calculate_top(include_str!("../test.txt"));
        assert_eq!(top, "CMZ");
    }
}
