use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

type Chemical = String;
type Amount = (usize, Chemical);
// hash map from output to number of them and list of inputs
type Reactions = HashMap<Chemical, (usize, Vec<Amount>)>;

pub fn reduce_reuired_chemicals<'a>(
    reactions: &'a Reactions,
    spare_chemicals: &mut HashMap<&'a str, usize>,
    required_chemicals: HashMap<&'a str, usize>,
) -> HashMap<&'a str, usize> {
    let mut new_required_chemicals: HashMap<&str, usize> = HashMap::new();
    let mut new_spare: HashMap<&str, usize> = HashMap::new();
    for (chemical, amount) in required_chemicals.iter() {
        if chemical == &"ORE" {
            *new_required_chemicals.entry(&chemical).or_insert(0) += amount;
            continue;
        }
        let mut required_amount = *amount;
        if let Some(spare_amount) = spare_chemicals.get_mut(chemical) {
            if required_amount > *spare_amount {
                required_amount -= *spare_amount;
                *spare_amount = 0;
            } else {
                required_amount = 0;
                *spare_amount -= required_amount;
            }
        }
        let inputs = reactions.get(*chemical).unwrap();
        let amount_provided = inputs.0;
        let reactions = (required_amount as f64 / amount_provided as f64).ceil() as usize;

        if reactions > 0 {
            let spare = reactions * amount_provided - required_amount;

            if spare > 0 {
                *new_spare.entry(chemical).or_insert(0) += spare;
            }

            for input in inputs.1.iter() {
                let required_input = new_required_chemicals.entry(&input.1).or_insert(0);
                *required_input += input.0 * reactions;
            }
        }
    }

    *spare_chemicals = new_spare;

    spare_chemicals.retain(|_chem, amount| *amount > 0);

    new_required_chemicals
}

pub fn get_number_of_ore(reactions: &Reactions, fuel: usize) -> usize {
    let mut spare_chemicals: HashMap<&str, usize> = HashMap::new();
    let mut required_chemicals: HashMap<&str, usize> = HashMap::new();
    let inputs = reactions.get("FUEL").unwrap();
    for input in inputs.1.iter() {
        let required_input = required_chemicals.entry(&input.1).or_insert(0);
        *required_input += input.0 * fuel;
    }

    loop {
        required_chemicals =
            reduce_reuired_chemicals(reactions, &mut spare_chemicals, required_chemicals);
        if required_chemicals.keys().count() <= 1 {
            return *required_chemicals.get("ORE").unwrap();
        }
    }
}

pub fn get_maximum_fuel_per_trillion_ore(reactions: &Reactions) -> usize {
    let min_ore = get_number_of_ore(reactions, 1);

    let max_ore = 1_000_000_000_000;

    let mut lower_fuel = 1_000_000_000_000 / min_ore;
    let mut upper_fuel = lower_fuel * 2;

    let mut max_fuel = lower_fuel;

    while lower_fuel <= upper_fuel {
        let mid_fuel = (upper_fuel + lower_fuel) / 2;
        if get_number_of_ore(reactions, mid_fuel) <= max_ore {
            max_fuel = mid_fuel;
            lower_fuel = mid_fuel + 1
        } else {
            upper_fuel = mid_fuel - 1;
        }
    }

    max_fuel
}

pub fn parse_amount(input: &str) -> Amount {
    let mut n_chemical = input.split_whitespace();
    let n = n_chemical.next().unwrap().trim().parse::<usize>().unwrap();
    let chemical = n_chemical.next().unwrap().trim().to_owned();
    (n, chemical)
}

pub fn parse_reactions(input: &str) -> Reactions {
    let mut result = HashMap::new();
    for line in input.lines() {
        let mut inout = line.split("=>");
        let inputs = inout
            .next()
            .unwrap()
            .split(",")
            .map(|x| parse_amount(x.trim()))
            .collect();
        let output = parse_amount(inout.next().unwrap().trim());
        if let Some(reaction) = result.get(&output.1) {
            panic!("double entries for {:?}", reaction);
        }
        result.insert(output.1, (output.0, inputs));
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let reactions = parse_reactions(&contents);

    let result = get_number_of_ore(&reactions, 1);
    println!("result part1: {}", result);

    let result = get_maximum_fuel_per_trillion_ore(&reactions);
    println!("result part2: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const REACTIONS_1: &str = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
    const REACTIONS_2: &str = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

    const REACTIONS_3: &str = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    const REACTIONS_4: &str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

    const REACTIONS_5: &str = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

    #[test]
    fn test_parse_amount() {
        let input = "9 ORE";
        let (n, chemical) = parse_amount(input);
        assert_eq!(n, 9);
        assert_eq!(chemical, "ORE");
    }

    #[test]
    fn test_parse_reactions() {
        let input = "10 ORE => 10 A
1 ORE => 1 B";
        let reactions = parse_reactions(input);
        let mut expected = HashMap::new();
        expected.insert("A".to_owned(), (10, vec![(10, "ORE".to_owned())]));
        expected.insert("B".to_owned(), (1, vec![(1, "ORE".to_owned())]));
        assert_eq!(reactions, expected);
    }

    #[test]
    fn test_get_number_of_ore_1() {
        let reactions = parse_reactions(REACTIONS_1);
        let ore = get_number_of_ore(&reactions, 1);
        assert_eq!(ore, 31);
    }
    #[test]
    fn test_get_number_of_ore_2() {
        let reactions = parse_reactions(REACTIONS_2);
        let ore = get_number_of_ore(&reactions, 1);
        assert_eq!(ore, 165);
    }
    #[test]
    fn test_get_number_of_ore_3() {
        let reactions = parse_reactions(REACTIONS_3);
        let ore = get_number_of_ore(&reactions, 1);
        assert_eq!(ore, 13312);
    }
    #[test]
    fn test_get_number_of_ore_4() {
        let reactions = parse_reactions(REACTIONS_4);
        let ore = get_number_of_ore(&reactions, 1);
        assert_eq!(ore, 180697);
    }
    #[test]
    fn test_get_number_of_ore_5() {
        let reactions = parse_reactions(REACTIONS_5);
        let ore = get_number_of_ore(&reactions, 1);
        assert_eq!(ore, 2210736);
    }

    #[test]
    fn test_fuel_for_max_ore_3() {
        let reactions = parse_reactions(REACTIONS_3);
        let fuel = get_maximum_fuel_per_trillion_ore(&reactions);
        assert_eq!(fuel, 82892753);
    }
}
