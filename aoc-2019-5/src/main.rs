use intcode_computer::Machine;

fn part_1(code: Vec<i32>) {
    // from the puzzle description
    let mut machine = Machine::new(code);
    println!("Part 1:");
    let _ = machine.run_with_input(1);
    println!("");
}

fn part_2(code: Vec<i32>) {
    let mut machine = Machine::new(code.clone());
    println!("Part 2:");
    let _ = machine.run_with_input(5);
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
