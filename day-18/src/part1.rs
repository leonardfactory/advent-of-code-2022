use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Droplet(pub i32, pub i32, pub i32);

pub fn parse_droplets(input: &str) -> HashSet<Droplet> {
    let mut droplets = HashSet::new();
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();
            Droplet(x, y, z)
        })
        .for_each(|droplet| {
            droplets.insert(droplet);
        });

    droplets
}

pub const COORDS: &[(i32, i32, i32)] = &[
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
    (-1, 0, 0),
    (1, 0, 0),
];

fn count_exposed(droplet: &Droplet, droplets: &HashSet<Droplet>) -> usize {
    COORDS
        .iter()
        .filter(|&&(x, y, z)| {
            !droplets.contains(&Droplet(droplet.0 + x, droplet.1 + y, droplet.2 + z))
        })
        .count()
}

pub fn count_droplets(droplets: &HashSet<Droplet>) -> usize {
    droplets
        .iter()
        .map(|droplet| count_exposed(droplet, droplets))
        .sum()
}

pub fn exposed_surface(input: &str) -> usize {
    let droplets = parse_droplets(input);
    count_droplets(&droplets)
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_example() {
        let test = "1,1,1\n2,1,1";
        assert_eq!(exposed_surface(test), 10);

        let input = include_str!("../test.txt");
        assert_eq!(exposed_surface(input), 64);
    }
}
