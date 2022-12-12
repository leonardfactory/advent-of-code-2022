use itertools::Itertools;

use crate::part1::{parse_input, Monkey};

pub fn count_unmanageable_monkey_business(input: &str, rounds: usize) -> usize {
    let mut monkeys = parse_input(input);
    for _ in 0..rounds {
        run_round(&mut monkeys);
        // debug_round(i, &monkeys);
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
    let factors: u64 = monkeys.iter().map(|m| m.test).product();

    for i in 0..monkeys.len() {
        monkeys[i].inspected += monkeys[i].items.len();

        while let Some(item) = monkeys[i].items.pop() {
            let item = monkeys[i].operation.apply(item) % factors;

            let (negative, positive) = (monkeys[i].negative, monkeys[i].positive);

            match item % monkeys[i].test {
                0 => monkeys[positive].items.push(item),
                _ => monkeys[negative].items.push(item),
            }
        }
    }
}

#[allow(dead_code)]
fn debug_round(i: usize, monkeys: &Vec<Monkey>) {
    println!("==== Round: {} ====", i);
    (0..monkeys.len()).for_each(|i| {
        println!("Monkey {}: {:?}", i, monkeys[i].items);
    });
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(
            count_unmanageable_monkey_business(input, 10_000),
            2713310158
        );
    }
}
