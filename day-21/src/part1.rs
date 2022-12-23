use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Monkey {
    Value(i64),
    Op(Operation, usize, usize),
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Mul,
    Sub,
    Div,
}

#[derive(Debug)]
pub struct MathTree {
    pub monkeys: Vec<Monkey>,
    pub names: HashMap<String, usize>,
}

fn parse_expression(input: &str) -> Captures {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\w+): ((\w+) ([*+/-]) (\w+)|(\d+))").unwrap();
    }
    RE.captures(input).unwrap()
}

fn monkey_index_by_name(
    monkeys: &mut Vec<Monkey>,
    names: &mut HashMap<String, usize>,
    name: String,
) -> usize {
    match names.get(&name) {
        Some(index) => *index,
        None => {
            let index = monkeys.len();
            monkeys.push(Monkey::Empty);
            names.insert(name, index);
            index
        }
    }
}

fn register_monkey_by_name(
    monkeys: &mut Vec<Monkey>,
    names: &mut HashMap<String, usize>,
    name: String,
    monkey: Monkey,
) {
    match names.get(&name) {
        Some(index) => {
            monkeys[*index] = monkey;
        }
        None => {
            let index = monkeys.len();
            names.insert(name, index);
            monkeys.push(monkey);
        }
    }
}

impl MathTree {
    pub fn parse(input: &str) -> Self {
        let mut monkeys = Vec::new();
        let mut names: HashMap<String, usize> = HashMap::new();

        input.lines().for_each(|line| {
            let captures = parse_expression(line);
            let name = captures.get(1).unwrap().as_str().to_owned();
            // println!("Captures: {:?}", captures);
            if captures.get(3).is_none() {
                let monkey = Monkey::Value(captures.get(6).unwrap().as_str().parse().unwrap());
                register_monkey_by_name(&mut monkeys, &mut names, name, monkey);
            } else {
                let left_name = captures.get(3).unwrap().as_str().to_owned();
                let right_name = captures.get(5).unwrap().as_str().to_owned();

                let left_index = monkey_index_by_name(&mut monkeys, &mut names, left_name);
                let right_index = monkey_index_by_name(&mut monkeys, &mut names, right_name);

                let monkey = Monkey::Op(
                    match captures.get(4).unwrap().as_str() {
                        "+" => Operation::Add,
                        "-" => Operation::Sub,
                        "*" => Operation::Mul,
                        "/" => Operation::Div,
                        _ => panic!("Unknown operation"),
                    },
                    left_index,
                    right_index,
                );

                register_monkey_by_name(&mut monkeys, &mut names, name, monkey);
            }
        });

        Self { monkeys, names }
    }
}

pub fn resolve_expression(tree: &MathTree, current: usize) -> i64 {
    match tree.monkeys[current] {
        Monkey::Value(value) => value,
        Monkey::Op(operation, left, right) => {
            let left_value = resolve_expression(tree, left);
            let right_value = resolve_expression(tree, right);

            match operation {
                Operation::Add => left_value + right_value,
                Operation::Sub => left_value - right_value,
                Operation::Mul => left_value * right_value,
                Operation::Div => left_value / right_value,
            }
        }
        Monkey::Empty => panic!("Empty value at {}", current),
    }
}

pub fn parse_and_solve(input: &str) -> i64 {
    let math_tree = MathTree::parse(input);
    resolve_expression(&math_tree, math_tree.names["root"])
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(parse_and_solve(input), 152);
    }

    #[test]
    fn test_ordering() {
        let input = "root: c + c\nc: a + b\na: 1\nb: 2";
        assert_eq!(parse_and_solve(input), 6);
    }
}
