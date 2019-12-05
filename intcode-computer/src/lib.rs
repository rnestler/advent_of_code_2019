use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Machine {
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

    pub fn read_code<P: AsRef<Path>>(path: P) -> Result<Vec<i32>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents
            .trim()
            .split(',')
            .map(|v| i32::from_str_radix(v, 10).expect("No integer"))
            .collect())
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
                let out = self.state[self.pc + 1] as usize;
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

            // jump-if-true
            5 => {
                let in1 = self.get_param(mode[0], self.state[self.pc + 1]);
                let in2 = self.get_param(mode[1], self.state[self.pc + 2]);
                if in1 != 0 {
                    assert!(in2 >= 0, "address out of range");
                    self.pc = in2 as usize;
                } else {
                    self.pc += 3;
                }
                None
            }
            // jump-if-false
            6 => {
                let in1 = self.get_param(mode[0], self.state[self.pc + 1]);
                let in2 = self.get_param(mode[1], self.state[self.pc + 2]);
                if in1 == 0 {
                    assert!(in2 >= 0, "address out of range");
                    self.pc = in2 as usize;
                } else {
                    self.pc += 3;
                }
                //
                None
            }
            // less than
            7 => {
                let in1 = self.get_param(mode[0], self.state[self.pc + 1]);
                let in2 = self.get_param(mode[1], self.state[self.pc + 2]);
                assert!(mode[2] == 0, "wrong mode");
                let out = self.state[self.pc + 3] as usize;

                if in1 < in2 {
                    self.state[out] = 1;
                } else {
                    self.state[out] = 0;
                }
                self.pc += 4;
                None
            }
            // equals
            8 => {
                let in1 = self.get_param(mode[0], self.state[self.pc + 1]);
                let in2 = self.get_param(mode[1], self.state[self.pc + 2]);
                assert!(mode[2] == 0, "wrong mode");
                let out = self.state[self.pc + 3] as usize;
                if in1 == in2 {
                    self.state[out] = 1;
                } else {
                    self.state[out] = 0;
                }
                self.pc += 4;
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
