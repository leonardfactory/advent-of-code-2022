use itertools::Itertools;

use crate::part1::Instruction;

pub struct Program {
    x: i32,
    current: usize,
    monitor: Vec<char>,
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn parse(input: &str) -> Self {
        let instructions = input.lines().flat_map(Instruction::new).collect_vec();
        Self {
            x: 1,
            current: 0,
            monitor: vec![' '; 240],
            instructions,
        }
    }

    pub fn run(&mut self) {
        let crt_x: i32 = (self.current % 40).try_into().unwrap();
        self.monitor[self.current] = if crt_x >= self.x - 1 && crt_x <= self.x + 1 {
            '#'
        } else {
            '.'
        };

        match self.instructions[self.current] {
            Instruction::Noop => {}
            Instruction::AddX(n) => {
                self.x += n;
            }
        }

        self.current += 1;
    }
}

pub fn render_crt(input: &str) {
    let mut program = Program::parse(input);
    for _ in 0..240 {
        program.run();
    }
    for line in program.monitor.chunks(40) {
        println!("{}", line.iter().collect::<String>());
    }
}

#[cfg(test)]
pub mod tests {
    use crate::part2::*;

    #[test]
    fn test_example() {
        let input = include_str!("../test.txt");
        render_crt(input);
        assert_eq!(1, 1);
    }
}
