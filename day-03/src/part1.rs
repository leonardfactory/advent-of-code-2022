use core::panic;

use itertools::Itertools;

// #[derive(Debug)]
// pub struct Rucksack {
//     pub data: Vec<char>,
//     // pub second: Vec<char>,
// }

// impl Rucksack {
//     fn parse(data: &str) -> Rucksack {
//         let parts = &data.chars().chunks(data.len() / 2);
//         let mut iter = parts.into_iter();
//         Rucksack {
//             first: iter.next().unwrap().collect_vec(),
//             second: iter.next().unwrap().collect_vec()
//         }
//     }
// }

pub fn find_common_letters(data: &str) -> char {
    let first = data[0..(data.len() / 2 + 1)].chars(); //.sorted();
    let second = data[(data.len() / 2)..].chars(); // .sorted();

    for (a, b) in first.cartesian_product(second) {
        if a == b {
            return a;
        }
    }

    panic!("No common letters found");
}

pub fn char_priority(char: char) -> usize {
    match char {
        char if ('a'..='z').contains(&char) => char as usize - 'a' as usize + 1,
        char if ('A'..='Z').contains(&char) => char as usize - 'A' as usize + 27,
        _ => panic!("Invalid char"),
    }
}

pub fn sum_rucksack_priorities(data: &str) -> usize {
    data.lines()
        .map(|line| char_priority(find_common_letters(line)))
        .sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_parse_rucksack() {
        assert_eq!(find_common_letters("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
    }

    #[test]
    fn test_example_priorities() {
        let priorities = sum_rucksack_priorities(include_str!("../test.txt"));
        assert_eq!(priorities, 157);
    }
}
