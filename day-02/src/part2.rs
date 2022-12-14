use crate::part1::Strategy;

impl Strategy {
    const ROCK: usize = 0;
    const PAPER: usize = 1;
    const SCISSORS: usize = 2;

    const LOSE: usize = 0;
    const DRAW: usize = 1;
    const WIN: usize = 2;
}

pub fn find_shape(strategy: &Strategy) -> usize {
    match (strategy.player1, strategy.player2) {
        (Strategy::ROCK, Strategy::WIN) => Strategy::PAPER,
        (Strategy::PAPER, Strategy::WIN) => Strategy::SCISSORS,
        (Strategy::SCISSORS, Strategy::WIN) => Strategy::ROCK,
        (Strategy::ROCK, Strategy::LOSE) => Strategy::SCISSORS,
        (Strategy::PAPER, Strategy::LOSE) => Strategy::ROCK,
        (Strategy::SCISSORS, Strategy::LOSE) => Strategy::PAPER,
        (p1, Strategy::DRAW) => p1,
        _ => panic!("Unknown strategy: {:?}", strategy),
    }
}

pub fn transform_strategy(strategy: &Strategy) -> Strategy {
    Strategy {
        player1: strategy.player1,
        player2: find_shape(strategy),
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        part1::{parse, winning_score},
        part2::*,
    };

    #[test]
    fn test_example() {
        let input: Vec<_> = parse(include_str!("../test.txt"))
            .iter()
            .map(transform_strategy)
            .collect();
        assert_eq!(winning_score(&input), 12);
    }
}
