pub fn top_three_elves(elves: &[u32]) -> u32 {
    elves.iter().rev().take(3).sum()
}

#[cfg(test)]
pub mod tests {
    use crate::{part1::order_elves, part2::*};

    #[test]
    fn test_example() {
        let elves = order_elves(include_str!("../test.txt"));
        assert_eq!(top_three_elves(&elves), 45000);
    }
}
