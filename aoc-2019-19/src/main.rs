use intcode_computer::{Machine, Pos};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = Machine::read_code("input.txt")?;

    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut machine = Machine::new(code.clone());
            machine.add_input(y);
            machine.add_input(x);
            machine.run_until_block();
            let output = machine.drain_output();
            assert_eq!(output.len(), 1);
            count += output[0];
        }
    }

    println!("Result part 1: {}", count);

    Ok(())
}
