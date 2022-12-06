pub fn order_elves(data: &str) -> Vec<u32> {
    let mut rows: Vec<u32> = data
        .split("\n\n")
        .map(|r| r.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect();

    rows.sort_unstable();
    rows
}

pub fn max_elf(elves: &[u32]) -> u32 {
    *elves.last().unwrap()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let elves = order_elves(include_str!("../test.txt"));
        assert_eq!(max_elf(&elves), 24000);
    }
}
