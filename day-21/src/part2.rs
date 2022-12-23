use std::collections::VecDeque;

use crate::part1::{resolve_expression, MathTree, Monkey};

impl MathTree {
    fn set_human(&mut self, value: i64) {
        let human_index = self.names["humn"];
        self.monkeys[human_index] = Monkey::Value(value);
    }

    fn has_uman(&self, index: usize) -> bool {
        match self.monkeys[index] {
            Monkey::Value(_) => self.names["humn"] == index,
            Monkey::Op(_, left, right) => self.has_uman(left) || self.has_uman(right),
            Monkey::Empty => panic!("Empty monkey"),
        }
    }
}

/**
 * Returns (fixed, human)
 */
fn comparing_values(tree: &MathTree) -> (usize, usize) {
    let (left, right) = match tree.monkeys[tree.names["root"]] {
        Monkey::Op(_, left, right) => (left, right),
        _ => panic!("Root is not an operation"),
    };

    match tree.has_uman(left) {
        true => (right, left),
        false => (left, right),
    }
}

const GRADIENT_START: i64 = 4_000_000_000_000;
const GRADIENT_DELTA: i64 = 1_000_000_000;

/**
 * An home made gradient descent algorithm.
 */
pub fn find_human_yell(input: &str) -> i64 {
    let mut tree = MathTree::parse(input);
    let (fixed_root, human_root) = comparing_values(&tree);
    let fixed_value = resolve_expression(&tree, fixed_root);
    // println!("Fixed value: {}", fixed_value);

    let mut score = |x: i64| {
        tree.set_human(x);
        let value = resolve_expression(&tree, human_root);
        value - fixed_value
    };

    let mut queue: VecDeque<(i64, i64, i64, i64)> = VecDeque::new();
    queue.push_back((
        -GRADIENT_START,
        -GRADIENT_START + GRADIENT_DELTA,
        score(-GRADIENT_START),
        score(-GRADIENT_START + GRADIENT_DELTA),
    ));

    while let Some((from, to, from_value, to_value)) = queue.pop_front() {
        let gradient = to_value - from_value;
        // println!(
        //     "({}..{}) = ({}, {}), gradient={}",
        //     from, to, from_value, to_value, gradient
        // );

        if from_value.signum() != to_value.signum() {
            // println!("Found inversion at ({}, {})", from, to);
            if to - from < 10_000 {
                for i in from..=to {
                    if score(i) == 0 {
                        return i;
                    }
                }
            } else {
                queue.push_front((
                    from,
                    from + (to - from) / 2,
                    from_value,
                    score(from + (to - from) / 2),
                ));
                queue.push_front((
                    from + (to - from) / 2,
                    to,
                    score(from + (to - from) / 2),
                    to_value,
                ));
            }
        }

        if gradient.signum() != to_value.signum() {
            queue.push_back((
                to,
                to + GRADIENT_DELTA,
                to_value,
                score(to + GRADIENT_DELTA),
            ));
        } else {
            queue.push_back((
                from - GRADIENT_DELTA,
                from,
                score(from - GRADIENT_DELTA),
                from_value,
            ));
        }
    }
    -1
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(find_human_yell(input), 301);
    }
}
