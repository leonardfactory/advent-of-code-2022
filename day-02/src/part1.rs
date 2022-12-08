#[derive(Debug)]
pub struct Strategy {
    pub player1: usize,
    pub player2: usize,
}

fn parse_value(data: &str) -> usize {
    match data {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!("Unknown value: {}", data),
    }
}

pub fn parse(data: &str) -> Vec<Strategy> {
    data.lines()
        .map(|l| {
            let mut parts = l.split(' ');
            Strategy {
                player1: parse_value(parts.next().unwrap()),
                player2: parse_value(parts.next().unwrap()),
            }
        })
        .collect()
}

// 0 Sasso
// 1 Carta
// 2 Forbice
pub fn strategy_score(strategy: &Strategy) -> usize {
    if strategy.player1 == strategy.player2 {
        return 3 + strategy.player2 + 1;
    }

    let is_win = match strategy.player2 {
        0 => strategy.player1 == 2,
        1 => strategy.player1 == 0,
        2 => strategy.player1 == 1,
        _ => panic!("Unknown value: {}", strategy.player2),
    };

    if is_win {
        return 6 + strategy.player2 + 1;
    }

    strategy.player2 + 1 // + 0
}

pub fn winning_score(strategies: &[Strategy]) -> usize {
    strategies.iter().map(strategy_score).sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_winner() {
        let strategy = Strategy {
            player1: 0,
            player2: 1,
        };
        assert_eq!(strategy_score(&strategy), 8);
    }

    #[test]
    fn test_example() {
        let strategies = parse(include_str!("../test.txt"));
        assert_eq!(winning_score(&strategies), 15);
    }
}
