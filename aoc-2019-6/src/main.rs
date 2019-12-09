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

fn get_orbit_map_2(content: &str) -> HashMap<&str, &str> {
    let mut orbit_map = HashMap::new();
    for line in content.lines() {
        let mut line_it = line.split(')');
        let mass = line_it.next().unwrap();
        let orbiter = line_it.next().unwrap().trim();

        orbit_map.insert(orbiter, mass);
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

fn get_transfer_path_to_com<'a>(
    orbit_map: &'a HashMap<&str, &str>,
    mut start: &'a str,
) -> Vec<&'a str> {
    let mut path = vec![];
    loop {
        let mass = *orbit_map.get(start).unwrap();
        path.push(mass);
        start = mass;
        if start == "COM" {
            break;
        }
    }
    path
}

fn get_orbital_transfers(content: &str) -> usize {
    let orbit_map = get_orbit_map_2(content);
    let my_transfer_path = get_transfer_path_to_com(&orbit_map, "YOU");
    let santa_transfer_path = get_transfer_path_to_com(&orbit_map, "SAN");
    for (pos_santa, mass) in santa_transfer_path.iter().enumerate() {
        if let Some(pos) = my_transfer_path.iter().position(|e| &e == &mass) {
            return pos + pos_santa;
        }
    }
    0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let result = get_total_number_of_orbits(&contents);
    println!("Result part 1: {}", result);

    println!("Result part 2: {}", get_orbital_transfers(&contents));

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

    #[test]
    fn test_get_orbital_transfers() {
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
K)L
K)YOU
I)SAN";
        assert_eq!(4, get_orbital_transfers(input));
    }
}
