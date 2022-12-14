use itertools::Itertools;

const CYCLES_COUNT: usize = 6;

pub fn signal_strength(input: &str) -> i32 {
    let mut program = Program::parse(input);
    println!("program instructions are {}", program.instructions.len());
    let strengths = (0..CYCLES_COUNT).map(|i| {
        let total_cycles = 20 + i * 40;
        let cycle = if i == 0 { 19 } else { 40 };
        program.run(cycle) * total_cycles as i32
    });

    strengths.sum()
}

/**
 * An instruction is a single line of the program.
 */
pub enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    pub fn new(input: &str) -> Vec<Self> {
        let mut tokens = input.split(' ');
        match tokens.next().unwrap() {
            "noop" => vec![Self::Noop],
            "addx" => vec![
                Self::Noop,
                Self::AddX(tokens.next().unwrap().parse().unwrap()),
            ],
            _ => panic!("Unknown instruction"),
        }
    }
}

/**
 * A program is a sequence of instructions.
 */
pub struct Program {
    x: i32,
    current: usize,
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn parse(input: &str) -> Self {
        let instructions = input.lines().flat_map(Instruction::new).collect_vec();
        Self {
            x: 1,
            current: 0,
            instructions,
        }
    }

    pub fn run(&mut self, cycles: usize) -> i32 {
        for _ in 0..cycles {
            if self.current >= self.instructions.len() {
                return self.x;
            }

            match self.instructions[self.current] {
                Instruction::Noop => {}
                Instruction::AddX(n) => {
                    self.x += n;
                }
            }
            self.current += 1;
        }

        println!("cycle={}: x={}", self.current, self.x);

        self.x
    }
}

#[cfg(test)]
pub mod tests {
    use crate::part1::*;

    // #[test]
    // fn test_first_cycle() {
    //     let input = include_str!("../test.txt");
    //     let mut program = Program::parse(input);
    //     assert_eq!(program.run(20), 21);
    // }

    #[test]
    fn test_small_program() {
        let input = "noop\naddx 3\naddx -5";
        let mut program = Program::parse(input);
        assert_eq!(program.run(1), 1);
        assert_eq!(program.run(1), 1);
        assert_eq!(program.run(1), 4);

        let mut program2 = Program::parse(input);
        assert_eq!(program2.run(3), 4);
    }

    #[test]
    fn test_sample_program() {
        let input = include_str!("../test.txt");
        assert_eq!(signal_strength(input), 13140);
    }
}
