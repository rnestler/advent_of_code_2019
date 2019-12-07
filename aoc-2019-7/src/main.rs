use intcode_computer::Machine;
use permute::permutations_of;

fn find_max_thruster_signal(code: Vec<i32>) -> i32 {
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
    let result = find_max_thruster_signal(code);
    println!("result part1: {}", result);
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
}
