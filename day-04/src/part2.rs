use crate::part1::Assignment;

pub fn is_partial_overlap(first: (usize, usize), second: (usize, usize)) -> bool {
    (first.0 >= second.0 && first.0 <= second.1) || (first.1 >= second.0 && first.1 <= second.1)
}

pub fn has_partial_overlap(assignment: &Assignment) -> bool {
    is_partial_overlap(assignment.first, assignment.second)
        || is_partial_overlap(assignment.second, assignment.first)
}

#[cfg(test)]
pub mod tests {
    use crate::{part1::parse_line, part2::*};

    #[test]
    fn test_partial_overlap() {
        // Yes
        assert!(has_partial_overlap(&parse_line("2-8,3-7")));
        assert!(has_partial_overlap(&parse_line("6-6,4-6")));
        assert!(has_partial_overlap(&parse_line("5-7,7-9")));

        // no
        assert!(!has_partial_overlap(&parse_line("2-3,4-8")));
        assert!(!has_partial_overlap(&parse_line("2-4,6-8")));
    }

    #[test]
    fn test_example() {
        // include_str!("../test.txt");
        // assert_eq!(1, 2);
    }
}
