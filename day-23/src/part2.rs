use crate::part1::{parse_map, simulate_step};

pub fn find_stop_round(input: &str) -> usize {
    let mut map = parse_map(input);
    let mut round = 0;
    loop {
        let prev_map = map.clone();
        simulate_step(&mut map, round);

        if prev_map.tiles == map.tiles {
            return round + 1;
        }

        if round > 1_000_000_000 {
            panic!("Too many rounds");
        }

        round += 1;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(find_stop_round(input), 20);
    }
}
