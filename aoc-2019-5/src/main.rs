use std::fs::File;
use std::io::prelude::*;

struct Machine {
    pc: usize,
    state: Vec<i32>,
    input: i32,
    output: i32,
}

impl Machine {
    pub fn new(state: Vec<i32>) -> Machine {
        Machine {
            pc: 0,
            state,
            input: 0,
            output: 0,
        }
    }

    pub fn get_mode_digits(mut instruction: i32) -> [u8; 3] {
        instruction /= 100;
        let mut modes = [0u8; 3];
        for d in 0..3 {
            modes[d] = (instruction % 10) as u8;
            instruction /= 10;
        }
        modes
    }

    pub fn get_param(&self, mode: u8, value: i32) -> i32 {
        if mode == 0 {
            self.state[value as usize]
        } else if mode == 1 {
            value
        } else {
            panic!("Invalid mode");
        }
    }

    pub fn step(&mut self) -> Option<i32> {
        let instruction = self.state[self.pc];
        let op = instruction % 100;
        let mode = Self::get_mode_digits(instruction);
        match op {
            1 => {
                let in1 = self.get_param(mode[0], self.state[self.pc + 1]);
                let in2 = self.get_param(mode[1], self.state[self.pc + 2]);
                assert!(mode[2] == 0, "wrong mode");
                let out = self.state[self.pc + 3] as usize;
                self.state[out] = in1 + in2;
                self.pc += 4;
                None
            }
            2 => {
                let in1 = self.get_param(mode[0], self.state[self.pc + 1]);
                let in2 = self.get_param(mode[1], self.state[self.pc + 2]);
                assert!(mode[2] == 0, "wrong mode");
                let out = self.state[self.pc + 3] as usize;
                self.state[out] = in1 * in2;
                self.pc += 4;
                None
            }
            // input
            3 => {
                let out = self.state[self.pc + 3] as usize;
                assert!(mode[0] == 0, "wrong mode");
                self.state[out] = self.input;
                self.pc += 2;
                None
            }
            // output
            4 => {
                let in1 = self.get_param(mode[0], self.state[self.pc + 1]);
                self.output = in1;
                println!("Output: {} at {}", self.output, self.pc);
                self.pc += 2;
                None
            }
            99 => Some(self.state[0]),
            _ => None,
        }
    }

    pub fn run(&mut self, noun: i32, verb: i32) -> i32 {
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

    pub fn run_with_input(&mut self, input: i32) -> i32 {
        self.input = input;
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

fn part_1(code: Vec<i32>) {
    // from the puzzle description
    let mut machine = Machine::new(code);
    let _ = machine.run_with_input(1);
}

fn part_2(code: Vec<i32>) {
    let mut machine = Machine::new(code.clone());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let code: Vec<_> = contents
        .trim()
        .split(',')
        .map(|v| i32::from_str_radix(v, 10).expect("No integer"))
        .collect();

    part_1(code.clone());
    part_2(code.clone());

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_code() {
        input = vec![]
    }
}
