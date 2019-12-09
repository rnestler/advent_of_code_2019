use intcode_computer::Machine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = Machine::read_code("input.txt")?;

    let mut machine = Machine::new(code.clone());
    let _ = machine.run_with_input(1);
    let result = machine.drain_output();

    println!("result: {:?}", result);

    let mut machine = Machine::new(code);
    let _ = machine.run_with_input(2);
    let result = machine.drain_output();

    println!("result: {:?}", result);

    Ok(())
}
