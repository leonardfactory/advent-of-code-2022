use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub fn count_monkey_business(input: &str, rounds: usize) -> usize {
    let mut monkeys = parse_input(input);
    for _ in 0..rounds {
        run_round(&mut monkeys);
    }

    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn run_round(monkeys: &mut Vec<Monkey>) {
    for i in 0..monkeys.len() {
        monkeys[i].inspected += monkeys[i].items.len();

        while let Some(item) = monkeys[i].items.pop() {
            let item = monkeys[i].operation.apply(item) / 3;

            let (negative, positive) = (monkeys[i].negative, monkeys[i].positive);

            match item % monkeys[i].test {
                0 => monkeys[positive].items.push(item),
                _ => monkeys[negative].items.push(item),
            }
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    pub fn apply(&self, item: u64) -> u64 {
        match self {
            Operation::Add(n) => item + *n,
            Operation::Multiply(n) => item * *n,
            Operation::Square => item * item,
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<u64>,
    pub operation: Operation,
    pub test: u64,
    pub positive: usize,
    pub negative: usize,
    pub inspected: usize,
}

impl Monkey {
    pub fn new(data: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Starting items: ([^\n]*)\s+Operation: new = old (\*|\+) (old|\d+)\s+Test: divisible by (\d+)\s+If true: throw to monkey (\d+)\s+If false: throw to monkey (\d+)").unwrap();
        }
        let captures = RE.captures(data).unwrap();

        let operation = match (
            captures.get(2).unwrap().as_str(),
            captures.get(3).unwrap().as_str(),
        ) {
            ("+", n) => Operation::Add(n.parse().unwrap()),
            ("*", "old") => Operation::Square,
            ("*", n) => Operation::Multiply(n.parse().unwrap()),
            _ => panic!("Unknown operation"),
        };

        Self {
            items: captures
                .get(1)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect(),
            operation,
            test: captures.get(4).unwrap().as_str().parse().unwrap(),
            positive: captures.get(5).unwrap().as_str().parse::<usize>().unwrap(),
            negative: captures.get(6).unwrap().as_str().parse::<usize>().unwrap(),
            inspected: 0,
        }
    }
}

pub fn parse_input(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(Monkey::new).collect()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_parse_input() {
        let test = include_str!("../test.txt");
        let monkeys = parse_input(test);
        assert_eq!(monkeys.len(), 4);
    }

    #[test]
    fn test_rounding() {
        assert_eq!(1501 / 3_usize, 500);
        assert_eq!(1500 / 3_usize, 500);
        assert_eq!(1862 / 3_usize, 620);
    }

    #[test]
    fn test_example() {
        let test = include_str!("../test.txt");
        assert_eq!(count_monkey_business(test, 20), 10605);
    }
}
