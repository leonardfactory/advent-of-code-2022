use std::collections::HashMap;

use itertools::Itertools;

use crate::part1::Graph;

#[derive(Clone, Debug)]
struct Character {
    pub minutes: usize,
    pub valve_index: usize,
}

#[derive(Clone, Debug)]
struct TwoSolution {
    pub valves: HashMap<usize, usize>,
    pub me: Character,
    pub elephant: Character,
    pub pressure: usize,
}

#[allow(dead_code)]
fn print_two_solution(graph: &Graph, solution: TwoSolution) {
    let me = graph.valves[solution.me.valve_index].name.clone();
    let elephant = graph.valves[solution.elephant.valve_index].name.clone();
    println!(
        "Solution: me={}, elephant={}, pressure={}",
        me, elephant, solution.pressure
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

fn find_best_paths(
    graph: &Graph,
    solution: &TwoSolution,
    who: &Character,
) -> Vec<(usize, usize, i32)> {
    graph.minimum_paths[&who.valve_index]
        .iter()
        .filter(|(to, _, _)| !solution.valves.contains_key(to))
        .map(|&(to, _from, cost)| {
            (
                to,
                cost,
                graph.valves[to].flow as i32 * (who.minutes as i32 - cost as i32 - 1),
            )
        })
        .sorted_by_key(|(_, _, max_flow)| -max_flow)
        .filter(|(_, _, max_flow)| *max_flow > 0)
        .collect_vec()
}

fn find_solutions(graph: &Graph, best: &mut Option<TwoSolution>, solution: TwoSolution) {
    if best.is_none() || solution.pressure > best.as_ref().unwrap().pressure {
        *best = Some(solution.clone());
        print_two_solution(graph, best.clone().unwrap());
    }

    let me_paths = find_best_paths(graph, &solution, &solution.me);

    for (to, cost, max_flow) in me_paths {
        let mut next_solution = solution.clone();
        next_solution.me.valve_index = to;
        next_solution.me.minutes -= cost + 1;
        next_solution.valves.insert(to, next_solution.me.minutes);
        next_solution.pressure += max_flow as usize;

        if best.is_none() || next_solution.pressure > best.as_ref().unwrap().pressure {
            *best = Some(next_solution.clone());
            print_two_solution(graph, best.clone().unwrap());
        }

        let elephant_paths = find_best_paths(graph, &next_solution, &next_solution.elephant);
        for (to, cost, max_flow) in elephant_paths {
            let mut next_solution = next_solution.clone();
            next_solution.elephant.valve_index = to;
            next_solution.elephant.minutes -= cost + 1;
            next_solution
                .valves
                .insert(to, next_solution.elephant.minutes);
            next_solution.pressure += max_flow as usize;

            find_solutions(graph, best, next_solution);
        }
    }
}

pub fn max_pressure_in_two(input: &str) -> usize {
    let graph = Graph::parse(input);
    let mut best = None;
    let valve_index = graph.names_map["AA"];
    find_solutions(
        &graph,
        &mut best,
        TwoSolution {
            valves: HashMap::new(),
            elephant: Character {
                minutes: 26,
                valve_index,
            },
            me: Character {
                minutes: 26,
                valve_index,
            },
            pressure: 0,
        },
    );
    print_two_solution(&graph, best.clone().unwrap());
    best.unwrap().pressure
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(max_pressure_in_two(input), 1707);
    }
}
