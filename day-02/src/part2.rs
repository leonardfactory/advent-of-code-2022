use crate::part1::Strategy;

impl Strategy {
    const Rock: usize = 0;
    const Paper: usize = 1;
    const Scissors: usize = 2;

    const Lose: usize = 0;
    const Draw: usize = 1;
    const Win: usize = 2;
}

pub fn find_shape(strategy: &Strategy) -> usize {
    match (strategy.player1, strategy.player2) {
        (Strategy::Rock, Strategy::Win) => Strategy::Paper,
        (Strategy::Paper, Strategy::Win) => Strategy::Scissors,
        (Strategy::Scissors, Strategy::Win) => Strategy::Rock,
        (Strategy::Rock, Strategy::Lose) => Strategy::Scissors,
        (Strategy::Paper, Strategy::Lose) => Strategy::Rock,
        (Strategy::Scissors, Strategy::Lose) => Strategy::Paper,
        (p1, Strategy::Draw) => p1,
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
    use crate::{part2::*, part1::{parse, winning_score}};

    #[test]
    fn test_example() {
        let input: Vec<_> = parse(include_str!("../test.txt")).iter().map(transform_strategy).collect();
        assert_eq!(winning_score(&input), 12);
    }
}
