use intcode_computer::Machine;

fn part_1(code: Vec<i32>) {
    // from the puzzle description
    let mut machine = Machine::new(code);
    println!("result part1: {}", machine.run(12, 2));
}

fn part_2(code: Vec<i32>) {
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
    let code = Machine::read_code("input.txt")?;

    part_1(code.clone());
    part_2(code.clone());

    Ok(())
}
