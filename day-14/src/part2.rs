use crate::part1::{scan_rock_paths, Mat, Pos, Scan};

fn scan_get(scan: &Scan, pos: &Pos) -> Mat {
    match pos.y == scan.floor {
        true => Mat::Rock,
        false => scan.get(pos),
    }
}

fn emit_sand(scan: &mut Scan) -> Option<Pos> {
    let mut pos = Pos { x: 500, y: 0 };
    loop {
        if scan_get(scan, &pos.to(0, 1)) == Mat::Air {
            pos = pos.to(0, 1);
        } else if scan_get(scan, &pos.to(-1, 1)) == Mat::Air {
            pos = pos.to(-1, 1);
        } else if scan_get(scan, &pos.to(1, 1)) == Mat::Air {
            pos = pos.to(1, 1);
        } else {
            match scan_get(scan, &pos) {
                Mat::Air => {
                    scan.insert(pos, Mat::Sand);
                    return Some(pos);
                }
                _ => {
                    return None;
                }
            }
        }
    }
}

pub fn count_sands_with_floor(input: &str) -> usize {
    let mut scan = scan_rock_paths(input);
    scan.floor = scan.max.y + 2;

    let mut count = 0;
    while emit_sand(&mut scan).is_some() {
        count += 1;
    }
    // scan.print();
    count
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        assert_eq!(count_sands_with_floor(input), 93);
        // assert_eq!(1, 2);
    }
}
