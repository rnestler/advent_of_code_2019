use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn get_total_number_of_orbits(content: &str) -> u32 {
    let mut orbit_map = HashMap::new();
    let mut orbit_scored_map = HashMap::new();
    orbit_scored_map.insert("COM", 0u32);
    let mut total = 0;
    for line in content.lines() {
        let mut line_it = line.split(')');
        let src = line_it.next().unwrap();
        let target = line_it.next().unwrap().trim();

        let orbitees = orbit_map.entry(src).or_insert(vec![]);
        orbitees.push(target);
    }

    let mut sources: Vec<&str> = vec!["COM"];
    loop {
        let mut next_source = vec![];
        for src in sources.clone().iter() {
            let score = *orbit_scored_map.get(src).expect("Not found in map");
            match orbit_map.get(src) {
                None => continue,
                Some(orbitees) => {
                    for orbitee in orbitees.iter() {
                        orbit_scored_map.insert(orbitee, score + 1);
                        total += score + 1;
                        next_source.push(*orbitee);
                    }
                }
            }
        }
        if next_source.is_empty() {
            break;
        }
        sources = next_source;
    }

    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let result = get_total_number_of_orbits(&contents);
    println!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_total_number_of_orbits() {
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        assert_eq!(42, get_total_number_of_orbits(input));
    }
}
