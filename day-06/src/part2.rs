use itertools::Itertools;

pub fn message_position(stream: &str) -> usize {
    stream.as_bytes().windows(14).position(|w| w.iter().unique().collect_vec().len() == 14).unwrap() + 14
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example_message() {
        assert_eq!(message_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(message_position("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(message_position("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(message_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(message_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
