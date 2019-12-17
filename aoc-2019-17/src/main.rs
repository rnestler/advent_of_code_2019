use intcode_computer::{Machine, Pos};
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = Machine::read_code("input.txt")?;
    let mut machine = Machine::new(code);
    machine.run_until_block();
    let output: Vec<u8> = machine.drain_output().iter().map(|v| *v as u8).collect();

    let mut map: HashSet<Pos> = HashSet::new();
    let output = String::from_utf8(output).unwrap();
    for (y, line) in output.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Pos::new(x as i64, y as i64);
            match c {
                '#' => {
                    map.insert(pos);
                }
                '.' => continue,
                '^' => continue,
                '>' => continue,
                '<' => continue,
                'v' => continue,
                _ => panic!("Unexpected character {:?}", c),
            }
        }
        println!("{}", line);
    }

    let sum: i64 = map
        .iter()
        .map(|entry| {
            let above = Pos::new(entry.x, entry.y - 1);
            let left = Pos::new(entry.x - 1, entry.y);
            let right = Pos::new(entry.x + 1, entry.y);
            let below = Pos::new(entry.x, entry.y + 1);

            if map.contains(&above)
                && map.contains(&below)
                && map.contains(&left)
                && map.contains(&right)
            {
                println!("intersection at {:?}", entry);
                entry.x * entry.y
            } else {
                0
            }
        })
        .sum();

    println!("result part1: {}", sum);

    Ok(())
}
