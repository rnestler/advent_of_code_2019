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
                println!("Add");
                let in1 = self.state[self.pc + 1] as usize;
                let in2 = self.state[self.pc + 2] as usize;
                let out = self.state[self.pc + 3] as usize;
                self.state[out] = self.state[in1] + self.state[in2];
                self.pc += 4;
                None
            }
            2 => {
                println!("Mul");
                let in1 = self.state[self.pc + 1] as usize;
                let in2 = self.state[self.pc + 2] as usize;
                let out = self.state[self.pc + 3] as usize;
                self.state[out] = self.state[in1] * self.state[in2];
                self.pc += 4;
                None
            }
            99 => {
                println!("Done");
                Some(self.state[0])
            }
            _ => {
                println!("Unknown");
                None
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut code: Vec<_> = contents
        .trim()
        .split(',')
        .map(|v| u32::from_str_radix(v, 10).expect("No integer"))
        .collect();
    // from the puzzle description
    code[1] = 12;
    code[2] = 2;
    let mut machine = Machine::new(code);
    loop {
        match machine.step() {
            Some(i) => {
                println!("result: {}", i);
                break;
            }
            _ => {}
        }
    }
    Ok(())
}
