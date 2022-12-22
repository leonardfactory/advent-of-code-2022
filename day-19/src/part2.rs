use crate::part1::{find_solutions, parse_blueprints, Blueprint, Solution};

pub fn blueprint_production(blueprint: &Blueprint) -> usize {
    let mut best: Option<Solution> = None;
    find_solutions(Solution::new(32), &mut best, blueprint);
    println!("Best: {:?}", best.as_ref().unwrap().geode);
    best.unwrap().geode
}

pub fn multiply_production(input: &str) -> usize {
    let blueprints = parse_blueprints(input);
    blueprints
        .iter()
        .take(3)
        .map(blueprint_production)
        .product()
}

#[cfg(test)]
pub mod tests {
    use crate::{part1::parse_blueprints, part2::*};

    #[test]
    fn test_32_minutes() {
        let input = include_str!("../test.txt");
        let blueprints = parse_blueprints(input);
        assert_eq!(blueprint_production(&blueprints[0]), 56);
        // assert_eq!(blueprint_production(&blueprints[1]), 62);
    }
}
