use itertools::Itertools;

pub fn marker_position(stream: &str) -> usize {
    stream.as_bytes().windows(4).position(|w| w.iter().unique().collect_vec().len() == 4).unwrap() + 4
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example_marker() {
        assert_eq!(marker_position("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(marker_position("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(marker_position("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(marker_position("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(marker_position("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }
}
