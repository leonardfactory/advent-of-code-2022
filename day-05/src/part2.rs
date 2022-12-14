use itertools::Itertools;

use crate::part1::{parse, Move};

fn reorder(stacks: &mut [Vec<char>], rules: &[Move]) {
    for rule in rules {
        let stack_from_len = stacks[rule.from].len();
        let mut to_move = stacks[rule.from]
            .drain((stack_from_len - rule.amount)..)
            .collect_vec();
        stacks[rule.to].append(&mut to_move);
    }
}

pub fn calculate_top_9001(data: &str) -> String {
    let (mut stacks, rules) = parse(data);
    reorder(&mut stacks, &rules);
    let top = stacks.iter().map(|s| s.last().unwrap_or(&' ')).join("");
    top
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let top = calculate_top_9001(include_str!("../test.txt"));
        assert_eq!(top, "MCD");
    }
}
