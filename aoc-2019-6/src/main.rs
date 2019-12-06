use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn get_orbit_map(content: &str) -> HashMap<&str, Vec<&str>> {
    let mut orbit_map = HashMap::new();
    for line in content.lines() {
        let mut line_it = line.split(')');
        let mass = line_it.next().unwrap();
        let orbiter = line_it.next().unwrap().trim();

        let orbiters = orbit_map.entry(mass).or_insert(vec![]);
        orbiters.push(orbiter);
    }
    orbit_map
}

fn get_total_number_of_orbits(content: &str) -> u32 {
    let orbit_map = get_orbit_map(content);
    let mut orbit_scored_map = HashMap::new();
    orbit_scored_map.insert("COM", 0u32);

    let mut total = 0;
    let mut sources: Vec<&str> = vec!["COM"];
    loop {
        let mut next_source = vec![];
        for mass in sources.clone().iter() {
            let score = *orbit_scored_map.get(mass).expect("Not found in map");
            match orbit_map.get(mass) {
                None => continue,
                Some(orbiters) => {
                    for orbiter in orbiters.iter() {
                        orbit_scored_map.insert(orbiter, score + 1);
                        total += score + 1;
                        next_source.push(*orbiter);
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
