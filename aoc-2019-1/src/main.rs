use std::fs::File;
use std::io::prelude::*;

fn get_required_fuel(mass: u32) -> u32 {
    mass / 3 - 2
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let total: u32 = contents
        .lines()
        .map(|line| get_required_fuel(line.parse().expect("Parsing failed")))
        .sum();

    println!("result part1: {}", total);
    Ok(())
}
