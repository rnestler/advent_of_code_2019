use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Pos {
    pub const fn new(x: i64, y: i64) -> Self {
        Pos { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Machine {
    pc: usize,
    state: Vec<i64>,
    extended_state: HashMap<u128, i64>,
    relative_base: usize,
    input: Rc<RefCell<VecDeque<i64>>>,
    output: Rc<RefCell<VecDeque<i64>>>,
}

pub enum StepResult {
    Halt(i64),
    NeedsInput,
    Continue,
}

impl Machine {
    pub fn new(state: Vec<i64>) -> Machine {
        Machine {
            pc: 0,
            state,
            extended_state: HashMap::new(),
            relative_base: 0,
            input: Rc::new(RefCell::new(VecDeque::new())),
            output: Rc::new(RefCell::new(VecDeque::new())),
        }
    }

    pub fn new_with_in_out(
        state: Vec<i64>,
        input: Rc<RefCell<VecDeque<i64>>>,
        output: Rc<RefCell<VecDeque<i64>>>,
    ) -> Machine {
        Machine {
            pc: 0,
            state,
            extended_state: HashMap::new(),
            relative_base: 0,
            input,
            output,
        }
    }

    pub fn read_code<P: AsRef<Path>>(path: P) -> Result<Vec<i64>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(Self::parse_code(&contents))
    }

    pub fn parse_code(code: &str) -> Vec<i64> {
        code.trim()
            .split(',')
            .map(|v| i64::from_str_radix(v, 10).expect("No integer"))
            .collect()
    }

    pub fn get_mode_digits(mut instruction: i64) -> [u8; 3] {
        instruction /= 100;
        let mut modes = [0u8; 3];
        for d in 0..3 {
            modes[d] = (instruction % 10) as u8;
            instruction /= 10;
        }
        modes
    }

    pub fn get_param(&self, mode: u8, value: i64) -> i64 {
        if mode == 1 {
            return value;
        }
        let location = if mode == 0 {
            assert!(value >= 0, "negativ address");
            value as u128
        } else if mode == 2 {
            let location = value as i128 + self.relative_base as i128;
            assert!(location >= 0, "negativ address");
            location as u128
        } else {
            panic!("Invalid mode");
        };

        if location >= self.state.len() as u128 {
            *self.extended_state.get(&location).unwrap_or(&0)
        } else {
            self.state[location as usize]
        }
    }

    pub fn write_memory(&mut self, mode: u8, location: i64, value: i64) {
        let location = if mode == 0 {
            location as i128
        } else if mode == 2 {
            location as i128 + self.relative_base as i128
        } else {
            panic!("Invalid mode");
        };
        assert!(location >= 0, "negativ address");
        let location = location as u128;

        if self.state.len() as u128 <= location {
            self.extended_state.insert(location, value);
        } else {
            let location = location as usize;
            self.state[location] = value
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
                let out = self.state[self.pc + 3];
                self.write_memory(mode[2], out, in1 + in2);
                self.pc += 4;
                StepResult::Continue
            }
            2 => {
                let in1 = self.get_param(mode[0], self.state[self.pc + 1]);
                let in2 = self.get_param(mode[1], self.state[self.pc + 2]);
                let out = self.state[self.pc + 3];
                self.write_memory(mode[2], out, in1 * in2);
                self.pc += 4;
                StepResult::Continue
            }
            // input
            3 => {
                if self.input.borrow().is_empty() {
                    StepResult::NeedsInput
                } else {
                    let out = self.state[self.pc + 1];
                    let value = self.input.borrow_mut().pop_front().expect("input empty");
                    self.write_memory(mode[0], out, value);
                    self.pc += 2;
                    StepResult::Continue
                }
            }
            // output
            4 => {
                let in1 = self.get_param(mode[0], self.state[self.pc + 1]);
                self.output.borrow_mut().push_back(in1);
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
                let out = self.state[self.pc + 3];

                if in1 < in2 {
                    self.write_memory(mode[2], out, 1);
                } else {
                    self.write_memory(mode[2], out, 0);
                }
                self.pc += 4;
                StepResult::Continue
            }
            // equals
            8 => {
                let in1 = self.get_param(mode[0], self.state[self.pc + 1]);
                let in2 = self.get_param(mode[1], self.state[self.pc + 2]);
                let out = self.state[self.pc + 3];
                if in1 == in2 {
                    self.write_memory(mode[2], out, 1);
                } else {
                    self.write_memory(mode[2], out, 0);
                }
                self.pc += 4;
                StepResult::Continue
            }
            // relative base offset
            9 => {
                let in1 = self.get_param(mode[0], self.state[self.pc + 1]);
                self.relative_base = (self.relative_base as i64 + in1) as usize;
                self.pc += 2;
                StepResult::Continue
            }
            99 => StepResult::Halt(self.state[0]),
            opcode => {
                panic!("Unknown op code: {}", opcode);
            }
        }
    }

    pub fn add_input(&mut self, input: i64) {
        self.input.borrow_mut().push_back(input);
    }

    pub fn get_output(&self) -> i64 {
        self.output
            .borrow_mut()
            .pop_front()
            .expect("No output available")
    }

    pub fn drain_output(&mut self) -> Vec<i64> {
        self.output.borrow_mut().drain(..).collect()
    }

    pub fn set_state(&mut self, address: usize, value: i64) {
        self.state[address] = value;
    }

    pub fn run(&mut self, noun: i64, verb: i64) -> i64 {
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

    pub fn run_with_input(&mut self, input: i64) -> i64 {
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
    use super::*;

    #[test]
    fn test_day_9_quine() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let code = Machine::parse_code(&input);
        let mut machine = Machine::new(code.clone());
        let _ = machine.run_with_input(1);
        let output = machine.drain_output();
        assert_eq!(output, code);
    }

    #[test]
    fn test_day_9_16_digit_number() {
        let input = "1102,34915192,34915192,7,4,7,99,0";
        let code = Machine::parse_code(&input);
        let mut machine = Machine::new(code);
        let _ = machine.run_with_input(1);
        let output = machine.get_output();
        assert_eq!(format!("{}", output).len(), 16);
    }
    #[test]
    fn test_day_9_large_number() {
        let input = "104,1125899906842624,99";
        let code = Machine::parse_code(&input);
        let mut machine = Machine::new(code);
        let _ = machine.run_with_input(1);
        let output = machine.get_output();
        assert_eq!(output, 1125899906842624);
    }
}
