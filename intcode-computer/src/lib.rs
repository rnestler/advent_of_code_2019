use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Machine {
    pc: usize,
    state: Vec<i32>,
    input: Rc<RefCell<VecDeque<i32>>>,
    output: Rc<RefCell<VecDeque<i32>>>,
}

pub enum StepResult {
    Halt(i32),
    //    Output(i32),
    NeedsInput,
    Continue,
}

impl Machine {
    pub fn new(state: Vec<i32>) -> Machine {
        Machine {
            pc: 0,
            state,
            input: Rc::new(RefCell::new(VecDeque::new())),
            output: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    pub fn new_with_in_out(
        state: Vec<i32>,
        input: Rc<RefCell<VecDeque<i32>>>,
        output: Rc<RefCell<VecDeque<i32>>>,
    ) -> Machine {
        Machine {
            pc: 0,
            state,
            input,
            output,
        }
    }

    pub fn read_code<P: AsRef<Path>>(path: P) -> Result<Vec<i32>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(Self::parse_code(&contents))
    }

    pub fn parse_code(code: &str) -> Vec<i32> {
        code.trim()
            .split(',')
            .map(|v| i32::from_str_radix(v, 10).expect("No integer"))
            .collect()
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

    pub fn step(&mut self) -> StepResult {
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
                StepResult::Continue
            }
            2 => {
                let in1 = self.get_param(mode[0], self.state[self.pc + 1]);
                let in2 = self.get_param(mode[1], self.state[self.pc + 2]);
                assert!(mode[2] == 0, "wrong mode");
                let out = self.state[self.pc + 3] as usize;
                self.state[out] = in1 * in2;
                self.pc += 4;
                StepResult::Continue
            }
            // input
            3 => {
                if self.input.borrow().is_empty() {
                    StepResult::NeedsInput
                } else {
                    let out = self.state[self.pc + 1] as usize;
                    assert!(mode[0] == 0, "wrong mode");
                    self.state[out] = self.input.borrow_mut().pop_front().expect("input empty");
                    self.pc += 2;
                    StepResult::Continue
                }
            }
            // output
            4 => {
                let in1 = self.get_param(mode[0], self.state[self.pc + 1]);
                self.output.borrow_mut().push_back(in1);
                println!("Output: {} at {}", in1, self.pc);
                self.pc += 2;
                StepResult::Continue
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
                StepResult::Continue
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
                StepResult::Continue
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
                StepResult::Continue
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
                StepResult::Continue
            }
            99 => StepResult::Halt(self.state[0]),
            _ => StepResult::Continue,
        }
    }

    pub fn add_input(&mut self, input: i32) {
        self.input.borrow_mut().push_back(input);
    }

    pub fn get_output(&self) -> i32 {
        self.output
            .borrow_mut()
            .pop_front()
            .expect("No output available")
    }

    pub fn run(&mut self, noun: i32, verb: i32) -> i32 {
        self.state[1] = noun;
        self.state[2] = verb;
        loop {
            match self.step() {
                StepResult::Halt(i) => {
                    return i;
                }
                StepResult::NeedsInput => {
                    panic!("Needs input");
                }
                _ => {}
            }
        }
    }

    pub fn run_with_input(&mut self, input: i32) -> i32 {
        self.add_input(input);
        loop {
            match self.step() {
                StepResult::Halt(i) => {
                    return i;
                }
                StepResult::NeedsInput => {
                    panic!("Needs input");
                }
                _ => {}
            }
        }
    }

    pub fn run_until_block(&mut self) -> StepResult {
        loop {
            match self.step() {
                StepResult::Halt(i) => return StepResult::Halt(i),
                StepResult::NeedsInput => return StepResult::NeedsInput,
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
