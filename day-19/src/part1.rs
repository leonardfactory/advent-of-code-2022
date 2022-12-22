use itertools::Itertools;
use lazy_static::lazy_static;
use num::integer::div_ceil;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Recipe {
    pub clay: usize,
    pub ore: usize,
    pub obsidian: usize,
}

impl Recipe {
    pub fn parse(input: &str) -> Self {
        let cleaned = input.replace('.', "");
        let tokens = cleaned.split(" and ").collect::<Vec<_>>();
        let mut recipe = Self {
            clay: 0,
            ore: 0,
            obsidian: 0,
        };
        tokens.iter().for_each(|t| {
            let (quantity, name) = t.split_once(' ').unwrap();
            match name {
                "clay" => recipe.clay = quantity.parse().unwrap(),
                "ore" => recipe.ore = quantity.parse().unwrap(),
                "obsidian" => recipe.obsidian = quantity.parse().unwrap(),
                _ => panic!("Unknown material: {}", name),
            }
        });
        recipe
    }

    pub fn has_capacity(&self, solution: &Solution) -> bool {
        solution.ore >= self.ore && solution.clay >= self.clay && solution.obsidian >= self.obsidian
    }

    pub fn consume(&self, solution: &mut Solution) {
        solution.ore -= self.ore;
        solution.clay -= self.clay;
        solution.obsidian -= self.obsidian;
    }
}

pub struct Blueprint {
    pub id: usize,
    pub recipes: HashMap<RobotType, Recipe>,
}

impl Blueprint {
    pub fn parse(input: &str) -> Self {
        lazy_static! {
            static ref RECIPE_RE: Regex = Regex::new(r"Each (\w+) robot costs (.+)").unwrap();
        }

        let (code, robots) = input.split_once(": ").unwrap();
        let recipes = robots
            .split(". ")
            .map(|r| {
                let captures = RECIPE_RE.captures(r).unwrap();
                let robot_type = match captures.get(1).unwrap().as_str() {
                    "ore" => RobotType::Ore,
                    "clay" => RobotType::Clay,
                    "obsidian" => RobotType::Obsidian,
                    "geode" => RobotType::Geode,
                    name => panic!("Unknown robot type: {}", name),
                };
                let recipe = Recipe::parse(captures.get(2).unwrap().as_str());
                (robot_type, recipe)
            })
            .collect::<HashMap<RobotType, Recipe>>();

        Self {
            id: code.replace("Blueprint ", "").parse().unwrap(),
            recipes,
        }
    }

    #[allow(dead_code)]
    fn is_needed(&self, solution: &Solution, robot: RobotType) -> bool {
        let current = match robot {
            RobotType::Ore => solution.ore,
            RobotType::Clay => solution.clay,
            RobotType::Obsidian => solution.obsidian,
            RobotType::Geode => solution.geode,
        };
        current < 20
    }
}

// Find solution through backtracking
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Solution {
    pub ore: usize,
    pub clay: usize,
    pub obsidian: usize,
    pub geode: usize,
    pub robots: HashMap<RobotType, usize>,
    pub minutes: usize,
}

impl Solution {
    pub fn new(minutes: usize) -> Self {
        let mut robots = HashMap::new();
        robots.insert(RobotType::Ore, 1);
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            robots,
            minutes,
        }
    }

    fn is_needed(&self, blueprint: &Blueprint, material: RobotType) -> bool {
        if material == RobotType::Geode {
            return true;
        }

        let available = match material {
            RobotType::Ore => self.ore,
            RobotType::Clay => self.clay,
            RobotType::Obsidian => self.obsidian,
            _ => panic!("Unexpected material: {:?}", material),
        };

        // Weird optimizations!
        if available >= 20 {
            return false;
        }

        let max_material = blueprint
            .recipes
            .iter()
            .filter(|(robot_type, _)| **robot_type != material)
            .map(|(_, recipe)| match material {
                RobotType::Ore => recipe.ore,
                RobotType::Clay => recipe.clay,
                RobotType::Obsidian => recipe.obsidian,
                _ => panic!("Unexpected material: {:?}", material),
            })
            .max()
            .unwrap_or(0);

        max_material * self.minutes > available
    }

    fn minutes_to(&self, blueprint: &Blueprint, robot: RobotType) -> Option<usize> {
        let recipe = blueprint.recipes.get(&robot).unwrap();

        let materials = vec![
            (recipe.ore, self.ore, RobotType::Ore),
            (recipe.clay, self.clay, RobotType::Clay),
            (recipe.obsidian, self.obsidian, RobotType::Obsidian),
        ];

        // Minimum
        let minimums: Vec<_> = materials
            .iter()
            .map(|&(needed, available, robot_type)| {
                match (needed, self.robots.get(&robot_type).unwrap_or(&0)) {
                    (0, _) => Some(0),
                    (_, 0) => None,
                    (needed, robots) => {
                        Some(div_ceil(needed as i32 - available as i32, *robots as i32).max(0))
                    }
                }
            })
            .collect();

        match minimums.iter().any(|m| m.is_none()) {
            true => None,
            false => minimums.iter().filter_map(|&m| m.map(|x| x as usize)).max(),
        }
    }

    fn produce(&mut self, minutes: usize) {
        self.ore += self.robots.get(&RobotType::Ore).unwrap_or(&0) * minutes;
        self.clay += self.robots.get(&RobotType::Clay).unwrap_or(&0) * minutes;
        self.obsidian += self.robots.get(&RobotType::Obsidian).unwrap_or(&0) * minutes;
        self.geode += self.robots.get(&RobotType::Geode).unwrap_or(&0) * minutes;
        self.minutes -= minutes;
    }
}

pub fn find_solutions(solution: Solution, best: &mut Option<Solution>, blueprint: &Blueprint) {
    if solution.minutes == 0 {
        if best.is_none() || solution.geode > best.as_ref().unwrap().geode {
            println!("New best: {:?}", solution);
            *best = Some(solution);
        }
        return;
    }

    // println!("Current solution: {:?}", solution);

    // 1. Try with each robot
    blueprint
        .recipes
        .iter()
        .filter_map(|(&robot, recipe)| {
            solution
                .minutes_to(blueprint, robot)
                .map(|minutes| (robot, recipe, minutes))
        })
        .filter(|&(robot, _, minutes)| {
            // println!("Try {:?} robot in {} minutes", robot, minutes);
            minutes < solution.minutes && solution.is_needed(blueprint, robot)
        })
        .sorted_by(|(r1, _, _), (r2, _, _)| r2.cmp(r1))
        .for_each(|(robot, recipe, minutes)| {
            // println!(
            //     "[min {}] Produce {:?} robot in {} minutes ({:?})",
            //     solution.minutes, robot, minutes, solution
            // );

            let mut new_solution = solution.clone();
            new_solution.produce(minutes + 1);
            new_solution
                .robots
                .entry(robot)
                .and_modify(|quantity| *quantity += 1)
                .or_insert(1);
            recipe.consume(&mut new_solution);

            find_solutions(new_solution, best, blueprint);
        })
}

pub fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    let blueprints: Vec<_> = input.lines().map(Blueprint::parse).collect();
    blueprints
}

pub fn blueprint_quality_level(blueprint: &Blueprint) -> usize {
    let mut best: Option<Solution> = None;
    find_solutions(Solution::new(24), &mut best, blueprint);
    println!("Best: {:?}", best.as_ref().unwrap().geode);
    best.unwrap().geode * blueprint.id
}

pub fn sum_quality_levels(input: &str) -> usize {
    let blueprints = parse_blueprints(input);
    blueprints.iter().map(blueprint_quality_level).sum()
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_parse_recipe() {
        assert_eq!(
            Recipe::parse("12 ore"),
            Recipe {
                ore: 12,
                clay: 0,
                obsidian: 0
            }
        );
        assert_eq!(
            Recipe::parse("1 clay and 1 ore and 1 obsidian"),
            Recipe {
                clay: 1,
                ore: 1,
                obsidian: 1
            }
        );
    }

    #[test]
    fn test_parse_blueprints() {
        let input = include_str!("../test.txt");
        let blueprints: Vec<_> = input.lines().map(Blueprint::parse).collect();
        assert_eq!(blueprints.len(), 2);
        assert_eq!(blueprints[0].id, 1);
        assert_eq!(
            blueprints[0].recipes[&RobotType::Geode],
            Recipe {
                clay: 0,
                ore: 2,
                obsidian: 7
            }
        );
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        let blueprints = parse_blueprints(input);
        assert_eq!(blueprint_quality_level(&blueprints[0]), 9);
        assert_eq!(blueprint_quality_level(&blueprints[1]), 24);
    }
}
