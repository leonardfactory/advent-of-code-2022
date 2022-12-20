use std::collections::{HashSet, VecDeque};

use crate::part1::{parse_droplets, Droplet, COORDS};

pub struct Obsidian {
    droplets: HashSet<Droplet>,
    min: (i32, i32, i32),
    max: (i32, i32, i32),
}

impl Obsidian {
    fn parse(input: &str) -> Self {
        let droplets = parse_droplets(input);
        let mut min = (i32::MAX, i32::MAX, i32::MAX);
        let mut max = (i32::MIN, i32::MIN, i32::MIN);
        for droplet in droplets.iter() {
            min.0 = min.0.min(droplet.0);
            min.1 = min.1.min(droplet.1);
            min.2 = min.2.min(droplet.2);
            max.0 = max.0.max(droplet.0);
            max.1 = max.1.max(droplet.1);
            max.2 = max.2.max(droplet.2);
        }
        Self { droplets, min, max }
    }

    fn is_inside(&self, droplet: &Droplet) -> bool {
        droplet.0 >= self.min.0 - 1
            && droplet.0 <= self.max.0 + 1
            && droplet.1 >= self.min.1 - 1
            && droplet.1 <= self.max.1 + 1
            && droplet.2 >= self.min.2 - 1
            && droplet.2 <= self.max.2 + 1
    }
}

fn count_droplets(from: &Droplet, obsidian: &Obsidian) -> usize {
    let mut queue: VecDeque<Droplet> = VecDeque::new();
    let mut visited: HashSet<Droplet> = HashSet::new();
    let mut faces = 0;
    queue.push_back(*from);
    while let Some(droplet) = queue.pop_front() {
        if !obsidian.is_inside(&droplet) || visited.contains(&droplet) {
            continue;
        }

        visited.insert(droplet);
        COORDS.iter().for_each(|&(x, y, z)| {
            let neighbour = &Droplet(droplet.0 + x, droplet.1 + y, droplet.2 + z);
            if obsidian.droplets.contains(neighbour) {
                faces += 1;
            } else {
                queue.push_back(*neighbour);
            }
        })
    }

    faces
}

pub fn exposed_external_surface(input: &str) -> usize {
    let obsidian = Obsidian::parse(input);
    count_droplets(
        &Droplet(obsidian.min.0, obsidian.min.1, obsidian.min.2),
        &obsidian,
    )
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(exposed_external_surface(input), 58);
    }
}
