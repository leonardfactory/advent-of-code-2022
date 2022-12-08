fn parse_range(data: &str) -> (usize, usize) {
    let mut parts = data.split('-');
    let start = parts.next().unwrap().parse::<usize>().unwrap();
    let end = parts.next().unwrap().parse::<usize>().unwrap();
    (start, end)
}

pub struct Assignment {
    pub first: (usize, usize),
    pub second: (usize, usize),
}

pub fn parse_line(data: &str) -> Assignment {
    let mut elves = data.split(',').map(parse_range);
    let first = elves.next().unwrap();
    let second = elves.next().unwrap();

    Assignment {
        first,
        second,
    }
}

pub fn has_overlaps(assignment: &Assignment) -> bool {
    assignment.first.0 >= assignment.second.0 && assignment.first.1 <= assignment.second.1 ||
    assignment.second.0 >= assignment.first.0 && assignment.second.1 <= assignment.first.1
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_has_overlaps() {
        // Yes
        assert!(has_overlaps(&parse_line("2-8,3-7")));
        assert!(has_overlaps(&parse_line("6-6,4-6")));

        // no
        assert!(!has_overlaps(&parse_line("5-7,7-9")));
        assert!(!has_overlaps(&parse_line("2-6,4-8")));
        assert!(!has_overlaps(&parse_line("2-4,6-8")));
    }
}
