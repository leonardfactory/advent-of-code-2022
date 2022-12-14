use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    fn parse(coords: (&str, &str)) -> Self {
        Self {
            x: coords.0.parse().unwrap(),
            y: coords.1.parse().unwrap(),
        }
    }

    pub fn to(&self, x: i32, y: i32) -> Self {
        Self {
            x: (self.x as i32 + x) as usize,
            y: (self.y as i32 + y) as usize,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Mat {
    Rock,
    Sand,
    Air,
    Source,
}

#[derive(Debug, Default)]
pub struct Scan {
    pub map: HashMap<Pos, Mat>,
    pub min: Pos,
    pub max: Pos,
    pub floor: usize,
}

impl Scan {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            min: Pos { x: 500, y: 0 },
            max: Pos { x: 500, y: 0 },
            floor: 0,
        }
    }
    pub fn insert(&mut self, pos: Pos, mat: Mat) {
        self.map.insert(pos, mat);
        self.min.x = self.min.x.min(pos.x);
        self.min.y = self.min.y.min(pos.y);
        self.max.x = self.max.x.max(pos.x);
        self.max.y = self.max.y.max(pos.y);
    }

    pub fn get(&self, pos: &Pos) -> Mat {
        *self.map.get(pos).unwrap_or(&Mat::Air)
    }

    pub fn print(&self) {
        println!("Map (min={:?}) (max={:?}):", self.min, self.max);
        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                let pos = Pos { x, y };
                let mat = self.map.get(&pos).unwrap_or(&Mat::Air);
                match mat {
                    Mat::Rock => print!("#"),
                    Mat::Sand => print!("o"),
                    Mat::Air => print!("."),
                    Mat::Source => print!("+"),
                }
            }
            println!();
        }
    }
}

pub fn scan_rock_paths(input: &str) -> Scan {
    let mut scan = Scan::new();
    // scan.insert(Pos { x: 500, y: 0 }, Mat::Source);

    for line in input.lines() {
        let mut directions = line
            .split(" -> ")
            .map(|t| t.split_once(',').unwrap())
            .map(Pos::parse)
            .peekable();

        while let Some(dir) = directions.next() {
            let Pos { x, y } = dir;
            let next = directions.peek();

            if let Some(next) = next {
                let Pos { x: nx, y: ny } = *next;
                if x == nx {
                    for y in y.min(ny)..=y.max(ny) {
                        scan.insert(Pos { x, y }, Mat::Rock);
                    }
                } else {
                    for x in x.min(nx)..=x.max(nx) {
                        scan.insert(Pos { x, y }, Mat::Rock);
                    }
                }
            }
        }
    }
    scan
}

fn emit_sand(scan: &mut Scan) -> Option<Pos> {
    let mut pos = Pos { x: 500, y: 0 };
    while pos.y <= scan.max.y {
        if scan.get(&pos.to(0, 1)) == Mat::Air {
            pos = pos.to(0, 1);
        } else if scan.get(&pos.to(-1, 1)) == Mat::Air {
            pos = pos.to(-1, 1);
        } else if scan.get(&pos.to(1, 1)) == Mat::Air {
            pos = pos.to(1, 1);
        } else {
            scan.insert(pos, Mat::Sand);
            return Some(pos);
        }
    }
    None
}

pub fn count_sands_before_rest(input: &str) -> usize {
    let mut scan = scan_rock_paths(input);
    let mut count = 0;
    while emit_sand(&mut scan).is_some() {
        count += 1;
    }
    scan.print();
    count
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    #[test]
    fn test_scan_rock_paths() {
        let input = include_str!("../test.txt");
        let scan = scan_rock_paths(input);
        scan.print();
        assert_eq!(scan.map.len(), 20);
        assert_eq!(scan.min, Pos { x: 494, y: 0 });
        assert_eq!(scan.max, Pos { x: 503, y: 9 });
    }

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(count_sands_before_rest(input), 24);
    }
}
