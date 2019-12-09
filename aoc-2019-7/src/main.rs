use intcode_computer::{Machine, StepResult};
use permute::permutations_of;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

fn find_max_thruster_signal_with_feedback(code: Vec<i64>) -> i64 {
    let phases = [5, 6, 7, 8, 9];
    let mut max_output = 0;
    for permutation in permutations_of(&phases) {
        let channels = [
            Rc::new(RefCell::new(VecDeque::new())),
            Rc::new(RefCell::new(VecDeque::new())),
            Rc::new(RefCell::new(VecDeque::new())),
            Rc::new(RefCell::new(VecDeque::new())),
            Rc::new(RefCell::new(VecDeque::new())),
            Rc::new(RefCell::new(VecDeque::new())),
        ];
        let mut amplifiers = [
            Machine::new_with_in_out(code.clone(), channels[0].clone(), channels[1].clone()),
            Machine::new_with_in_out(code.clone(), channels[1].clone(), channels[2].clone()),
            Machine::new_with_in_out(code.clone(), channels[2].clone(), channels[3].clone()),
            Machine::new_with_in_out(code.clone(), channels[3].clone(), channels[4].clone()),
            Machine::new_with_in_out(code.clone(), channels[4].clone(), channels[0].clone()),
        ];

        for (phase, amplifier) in permutation.zip(amplifiers.iter_mut()) {
            amplifier.add_input(*phase);
        }
        amplifiers[0].add_input(0);
        loop {
            let mut done_count = 0;
            for amplifier in amplifiers.iter_mut() {
                match amplifier.run_until_block() {
                    StepResult::Halt(_) => {
                        done_count += 1;
                    }
                    _ => {}
                }
            }
            if done_count >= 5 {
                break;
            }
        }
        //for amplifier in amplifiers.iter_mut() {
        //    amplifier.run_with_input(input);
        //    input = amplifier.get_output();
        //}

        let output = amplifiers[4].get_output();
        if output > max_output {
            max_output = output;
        }
    }
    max_output
}

fn find_max_thruster_signal(code: Vec<i64>) -> i64 {
    let phases = [0, 1, 2, 3, 4];
    let mut max_output = 0;
    for permutation in permutations_of(&phases) {
        let mut amplifiers = [
            Machine::new(code.clone()),
            Machine::new(code.clone()),
            Machine::new(code.clone()),
            Machine::new(code.clone()),
            Machine::new(code.clone()),
        ];

        for (phase, amplifier) in permutation.zip(amplifiers.iter_mut()) {
            amplifier.add_input(*phase);
        }
        let mut input = 0;
        for amplifier in amplifiers.iter_mut() {
            amplifier.run_with_input(input);
            input = amplifier.get_output();
        }
        if input > max_output {
            max_output = input;
        }
    }
    max_output
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = Machine::read_code("input.txt")?;
    let result = find_max_thruster_signal(code.clone());
    println!("result part1: {}", result);

    let result = find_max_thruster_signal_with_feedback(code);
    println!("result part2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_thruster_signal() {
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let code = Machine::parse_code(&input);
        assert_eq!(43210, find_max_thruster_signal(code));
    }

    #[test]
    fn test_find_max_thruster_signal_with_feedback() {
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
                     27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let code = Machine::parse_code(&input);
        assert_eq!(139629729, find_max_thruster_signal_with_feedback(code));
    }
}
