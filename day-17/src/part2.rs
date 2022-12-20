use core::panic;
use std::cmp::{max, min};

use crate::part1::{get_rocks, parse_flow, Chamber, Flow, Rock};

fn run_iteration(
    rocks: &[Rock],
    flow: &[Flow],
    chamber: &mut Chamber,
    flow_index: &mut usize,
    i: i64,
    max_height: i64,
) -> i64 {
    let rock = &rocks[(i % rocks.len() as i64) as usize];
    let mut rock_x = 2;
    let mut rock_y = max_height + 3;

    let mut j = 0;
    loop {
        // chamber.print(Some((rock, rock_x, rock_y)));
        match j % 2 {
            // Move X
            0 => {
                let delta = flow[*flow_index % flow.len()];
                *flow_index += 1;
                let next_x = match delta {
                    Flow::Left => max(0, rock_x - 1),
                    Flow::Right => min(rock_x + 1, 7 - rock.width),
                };

                if chamber.overlaps(rock, next_x, rock_y) {
                    j += 1;
                    continue;
                } else {
                    rock_x = next_x;
                }
            }
            // Move Y
            1 => {
                if chamber.overlaps(rock, rock_x, rock_y - 1) {
                    break;
                }
                rock_y -= 1;
            }
            _ => panic!("Unknown j: {}", j),
        }
        j += 1;
    }

    // println!("Adding rock {:?} at ({}, {})", rock, rock_x, rock_y);
    chamber.add(rock, rock_x, rock_y);
    // chamber.print(None);
    max(max_height, rock_y + rock.height)
}

fn find_loop(
    count: i64,
    iterations: i64,
    rocks: &[Rock],
    flow: &[Flow],
    chamber: &mut Chamber,
    flow_index: &mut usize,
    max_height: &mut i64,
) -> (i64, i64, i64) {
    let chunks = count / iterations;
    let mut heights_first_loop = vec![0; iterations as usize];
    let mut flows_first_loop = vec![0; iterations as usize];
    // let max_height = max_height;

    for n in 0..chunks {
        println!(
            "Current max height: {},n ={},fi={},ri={}",
            max_height,
            n,
            *flow_index % (flow.len() * 2),
            (n * iterations) % rocks.len() as i64
        );
        for i in 0..iterations {
            let ni = (n * iterations) + i;
            *max_height = run_iteration(rocks, flow, chamber, flow_index, ni, *max_height);

            if n == 0 {
                heights_first_loop[i as usize] = *max_height;
                flows_first_loop[i as usize] = *flow_index % flow.len();
            } else if let Some(dy) = chamber.are_equal(heights_first_loop[i as usize], *max_height)
            {
                if *flow_index % flow.len() != flows_first_loop[i as usize] {
                    println!("Loop detected, but flow is different");
                    continue;
                }
                println!(
                    "Loop detected at {}, n={}, prev_height={}, dy={}",
                    i, n, heights_first_loop[i as usize], dy
                );
                chamber.print(None);
                chamber.print_at_y(heights_first_loop[i as usize]);
                return (ni, n, *max_height - heights_first_loop[i as usize]);
            }
        }
    }
    panic!("No loop found");
}

/**
 * 1000 iterations
 * flow_loop = 100
 * li = 4
 * ln = 2
 * loop_size = 2 * 100 = 200
 * remaining = 1000 - 200 - 4 = 796
 * remaining_loops = 796 / 200 = 3
 * to_calculate = 796 % 200 = 196
 */
pub fn compute_height_simulate(input: &str, count: i64) -> i64 {
    let rocks = get_rocks();
    let flow = parse_flow(input);
    println!("Flow len: {:?}", flow.len());
    let mut flow_index = 0;

    let mut chamber = Chamber::empty();
    // let mut spawn_y = 3;
    let mut max_height = 0_i64;

    let iterations = flow.len() as i64 * rocks.len() as i64 * 2;
    println!("Loop min iterations: {}", iterations);

    let (loop_index, loop_size, height_diff) = find_loop(
        count,
        iterations,
        &rocks,
        &flow,
        &mut chamber,
        &mut flow_index,
        &mut max_height,
    );

    let loop_iterations = loop_size * iterations;

    let remaining = count - loop_index - 1;
    let remaining_loops = remaining / loop_iterations;
    let to_calculate = remaining % loop_iterations;
    max_height += height_diff * remaining_loops;
    println!("Iterations: {}, count={}", iterations, count);
    println!(
        "Loop index: {}, size {}, rem={}, mh={}, loop-iter={}",
        loop_index, loop_size, remaining, max_height, loop_iterations
    );

    println!("To calculate: {} (h={})", to_calculate, height_diff);

    chamber.tiles.clone().iter().for_each(|(key, tile)| {
        chamber
            .tiles
            .insert((key.0, key.1 + height_diff * remaining_loops), *tile);
    });

    for i in 0..(to_calculate) {
        max_height = run_iteration(
            &rocks,
            &flow,
            &mut chamber,
            &mut flow_index,
            loop_index + 1 + i,
            max_height,
        );
    }

    max_height
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(
            compute_height_simulate(input, 1_000_000_000_000_i64),
            1514285714288
        );
    }
}
