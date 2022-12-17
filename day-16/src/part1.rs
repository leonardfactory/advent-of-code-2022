use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Valve {
    pub name: String,
    pub index: usize,
    pub flow: usize,
}

pub struct Graph {
    pub valves: Vec<Valve>,
    pub edges: HashMap<usize, Vec<usize>>,

    pub names_map: HashMap<String, usize>,
    // Minimum paths from a valve A to a valve B in minutes
    pub minimum_paths: HashMap<usize, Vec<(usize, usize, usize)>>,
}

impl Graph {
    pub fn parse(input: &str) -> Graph {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Valve (\w+) has flow rate=(\d+); tunnel(?:s?) lead(?:s?) to valve(?:s?) (.+)"
            )
            .unwrap();
        }

        let mut names_map = HashMap::new();
        let mut tunnels_map = HashMap::new();

        let valves = input
            .lines()
            .enumerate()
            .map(|(i, line)| {
                // println!("Parsing line {}", line);
                let captures = RE.captures(line).unwrap();
                let name = captures[1].to_string();
                let tunnels: Vec<_> = captures[3]
                    .trim()
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect();

                names_map.insert(name.clone(), i);
                tunnels_map.insert(name.clone(), tunnels);

                Valve {
                    name,
                    index: i,
                    flow: captures[2].parse().unwrap(),
                }
            })
            .collect_vec();

        let mut edges = HashMap::new();
        tunnels_map.iter().for_each(|(name, tunnels)| {
            let index = names_map[name];
            let valve_edges = tunnels.iter().map(|tunnel| names_map[tunnel]).collect_vec();
            edges.insert(index, valve_edges);
        });

        let minimum_paths = compute_minimum_paths(&valves, &edges);

        Graph {
            valves,
            edges,
            names_map,
            minimum_paths,
        }
    }

    pub fn get_by_name(&self, name: &str) -> &Valve {
        &self.valves[self.names_map[name]]
    }
}

fn compute_minimum_paths(
    valves: &[Valve],
    edges: &HashMap<usize, Vec<usize>>,
) -> HashMap<usize, Vec<(usize, usize, usize)>> {
    let mut minimum_paths = HashMap::new();
    for i in 0..valves.len() {
        let mut queue: VecDeque<_> = VecDeque::new();
        queue.push_back((i, None, 0));
        let mut visited: HashSet<usize> = HashSet::with_capacity(valves.len());
        let mut costs: HashMap<usize, (usize, usize)> = HashMap::with_capacity(valves.len());
        visited.insert(i);

        while let Some((current, from, cost)) = queue.pop_front() {
            for edge in edges[&current].iter() {
                if !visited.contains(edge) {
                    let from_edge = from.unwrap_or(*edge);
                    costs.insert(*edge, (from_edge, cost + 1));
                    visited.insert(*edge);
                    queue.push_back((*edge, Some(from.unwrap_or(*edge)), cost + 1));
                }
            }
        }

        minimum_paths.insert(i, costs.iter().map(|(k, v)| (*k, v.0, v.1)).collect_vec());
    }

    minimum_paths
}

#[derive(Clone, Debug)]
struct Solution {
    pub valves: HashMap<usize, usize>,
    pub minutes: usize,
    pub current: usize,
    pub pressure: usize,
}

#[allow(dead_code)]
fn print_solution(graph: &Graph, solution: Solution) {
    let current = graph.valves[solution.current].name.clone();
    println!(
        "Solution: current={}, pressure={}",
        current, solution.pressure
    );

    for (&valve, &minutes) in solution.valves.iter().sorted_by_key(|(_, m)| **m) {
        let valve_name = graph.valves[valve].name.clone();
        let valve_flow = graph.valves[valve].flow;
        println!(
            " - Valve {} opened for {} minutes (flow = {})",
            valve_name, minutes, valve_flow
        );
    }
}

fn find_solutions(graph: &Graph, best: &mut Option<Solution>, solution: Solution) {
    if best.is_none() || solution.pressure > best.as_ref().unwrap().pressure {
        // println!("\nCurrent best solution");
        // print_solution(graph, solution.clone());
        *best = Some(solution.clone());
    }

    let best_paths = graph.minimum_paths[&solution.current]
        .iter()
        .enumerate()
        .map(|(i, (to, from, cost))| (i, *to, *from, *cost))
        .filter(|(_, to, _, _)| !solution.valves.contains_key(to))
        .map(|(i, to, from, cost)| {
            (
                i,
                to,
                from,
                cost,
                graph.valves[to].flow as i32 * (solution.minutes as i32 - cost as i32 - 1),
            )
        })
        .sorted_by_key(|(_, _, _, _, max_flow)| -max_flow)
        .filter(|(_, _, _, _, max_flow)| *max_flow > 0)
        .collect_vec();

    for (_i, to, _edge, cost, max_flow) in best_paths {
        let mut next_solution = solution.clone();
        next_solution.current = to;
        next_solution.minutes -= cost + 1;
        next_solution.valves.insert(to, next_solution.minutes);
        next_solution.pressure += max_flow as usize;
        find_solutions(graph, best, next_solution);
    }
}

pub fn max_pressure(input: &str) -> usize {
    let graph = Graph::parse(input);
    let mut best = None;
    find_solutions(
        &graph,
        &mut best,
        Solution {
            valves: HashMap::new(),
            minutes: 30,
            current: graph.names_map["AA"],
            pressure: 0,
        },
    );
    // print_solution(&graph, best.clone().unwrap());
    best.unwrap().pressure
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(max_pressure(input), 1651);
    }
}
