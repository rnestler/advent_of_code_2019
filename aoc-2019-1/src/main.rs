use std::fs::File;
use std::io::prelude::*;

fn get_required_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn get_required_fuel_part2(mass: i32) -> i32 {
    let fuel = get_required_fuel(mass);
    if fuel <= 0 {
        0
    } else {
        fuel + get_required_fuel_part2(fuel)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let total: i32 = contents
        .lines()
        .map(|line| get_required_fuel(line.parse().expect("Parsing failed")))
        .sum();

    println!("result part1: {}", total);

    let total: i32 = contents
        .lines()
        .map(|line| get_required_fuel_part2(line.parse().expect("Parsing failed")))
        .sum();
    println!("result part2: {}", total);
    Ok(())
}
