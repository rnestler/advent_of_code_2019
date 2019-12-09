use intcode_computer::Machine;

fn part_1(code: Vec<i64>) {
    // from the puzzle description
    let mut machine = Machine::new(code);
    let _ = machine.run_with_input(1);
    let output = machine.drain_output();
    println!("Part 1: {:?}", output);
}

fn part_2(code: Vec<i64>) {
    let mut machine = Machine::new(code.clone());
    let _ = machine.run_with_input(5);
    let output = machine.get_output();
    println!("Part 2: {}", output);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = Machine::read_code("input.txt")?;

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
