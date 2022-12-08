use std::collections::HashSet;

use itertools::Itertools;

use crate::part1::char_priority;

pub fn sum_badges_priorities(data: &str) -> usize {
    data.lines()
        .chunks(3)
        .into_iter()
        .map(|lines| {
            let uniques: Vec<_> = lines.map(find_uniq_letters).collect();
            let chars = shared_letters(&uniques[0], &shared_letters(&uniques[1], &uniques[2]));
            char_priority(*chars.iter().next().unwrap())
        })
        .sum()
}

pub fn find_uniq_letters(line: &str) -> HashSet<char> {
    HashSet::from_iter(line.chars().unique())
}

pub fn shared_letters(sack: &HashSet<char>, other: &HashSet<char>) -> HashSet<char> {
    sack.iter().filter(|c| other.contains(c)).cloned().collect()
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let priorities = sum_badges_priorities(include_str!("../test.txt"));
        assert_eq!(priorities, 70);
    }
}
