use std::fs::File;
use std::io::prelude::*;

struct Machine {
    pc: usize,
    state: Vec<u32>,
}

impl Machine {
    pub fn new(state: Vec<u32>) -> Machine {
        Machine { pc: 0, state }
    }

    pub fn step(&mut self) -> Option<u32> {
        match self.state[self.pc] {
            1 => {
                let in1 = self.state[self.pc + 1] as usize;
                let in2 = self.state[self.pc + 2] as usize;
                let out = self.state[self.pc + 3] as usize;
                self.state[out] = self.state[in1] + self.state[in2];
                self.pc += 4;
                None
            }
            2 => {
                let in1 = self.state[self.pc + 1] as usize;
                let in2 = self.state[self.pc + 2] as usize;
                let out = self.state[self.pc + 3] as usize;
                self.state[out] = self.state[in1] * self.state[in2];
                self.pc += 4;
                None
            }
            99 => Some(self.state[0]),
            _ => None,
        }
    }

    pub fn run(&mut self, noun: u32, verb: u32) -> u32 {
        self.state[1] = noun;
        self.state[2] = verb;
        loop {
            match self.step() {
                Some(i) => {
                    return i;
                }
                _ => {}
            }
        }
    }
}

fn part_1(code: Vec<u32>) {
    // from the puzzle description
    let mut machine = Machine::new(code);
    println!("result part1: {}", machine.run(12, 2));
}

fn part_2(code: Vec<u32>) {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut machine = Machine::new(code.clone());
            let result = machine.run(noun, verb);
            if result == 19690720 {
                println!("result part2: {}", 100 * noun + verb);
                return;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let code: Vec<_> = contents
        .trim()
        .split(',')
        .map(|v| u32::from_str_radix(v, 10).expect("No integer"))
        .collect();

    part_1(code.clone());
    part_2(code.clone());

    Ok(())
}
