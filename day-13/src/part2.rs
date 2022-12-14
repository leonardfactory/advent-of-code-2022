use crate::part1::{compare_pair, parse_pairs, Value};

pub fn decoder_packet_indices(input: &str) -> usize {
    let pairs = parse_pairs(input);
    let mut pairs = pairs
        .iter()
        .flat_map(|(p0, p1)| vec![p0, p1])
        .collect::<Vec<_>>();

    let packet2 = Value::List(vec![Value::List(vec![Value::Int(2)])]);
    let packet6 = Value::List(vec![Value::List(vec![Value::Int(6)])]);
    pairs.extend(vec![&packet2, &packet6]);

    pairs.sort_by(|&a, &b| compare_pair(&(a.clone(), b.clone())));

    let index2 = pairs.iter().position(|&p| p == &packet2).unwrap() + 1;
    let index6 = pairs.iter().position(|&p| p == &packet6).unwrap() + 1;

    index2 * index6
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(decoder_packet_indices(input), 140);
    }
}
