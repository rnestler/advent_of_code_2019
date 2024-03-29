use num::Integer;
use std::collections::HashMap;
type Vec3 = [i32; 3];

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Moon {
    pub pos: Vec3,
    pub velocity: Vec3,
}

impl Moon {
    pub const fn new(pos: Vec3) -> Moon {
        Moon {
            pos,
            velocity: [0, 0, 0],
        }
    }
    pub fn update(&mut self) {
        self.pos[0] += self.velocity[0];
        self.pos[1] += self.velocity[1];
        self.pos[2] += self.velocity[2];
    }

    pub fn get_potential_engergy(&self) -> i32 {
        self.pos.iter().map(|v| v.abs()).sum()
    }

    pub fn get_kinetic_engergy(&self) -> i32 {
        self.velocity.iter().map(|v| v.abs()).sum()
    }

    pub fn get_total_energy(&self) -> i32 {
        self.get_potential_engergy() * self.get_kinetic_engergy()
    }

    pub fn apply_gravity(&mut self, other: &mut Moon) {
        for i in 0..3 {
            let signum = (self.pos[i] - other.pos[i]).signum();
            self.velocity[i] -= signum;
            other.velocity[i] += signum;
        }
    }
}

fn simulation_step(moons: &mut [Moon]) {
    // update gravity
    for i in 0..moons.len() {
        let mut moon1 = moons[i].clone();
        for j in i + 1..moons.len() {
            moon1.apply_gravity(&mut moons[j]);
        }
        moons[i] = moon1;
    }
    // update velocity
    for moon in moons.iter_mut() {
        moon.update();
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State {
    pos_coordinate: [i32; 4],
    velocity_coordinate: [i32; 4],
}

impl State {
    pub fn from_moons(moons: &[Moon; 4], index: usize) -> State {
        let mut pos_coordinate = [0i32; 4];
        let mut velocity_coordinate = [0i32; 4];
        moons.iter().enumerate().for_each(|(i, moon)| {
            pos_coordinate[i] = moon.pos[index];
            velocity_coordinate[i] = moon.velocity[index];
        });
        State {
            pos_coordinate,
            velocity_coordinate,
        }
    }
}

fn find_loop(mut moons: [Moon; 4]) -> usize {
    let mut found_x = 0;
    let mut found_y = 0;
    let mut found_z = 0;
    let mut existing_states_x: HashMap<State, usize> = HashMap::new();
    let mut existing_states_y: HashMap<State, usize> = HashMap::new();
    let mut existing_states_z: HashMap<State, usize> = HashMap::new();
    for step in 0.. {
        simulation_step(&mut moons);
        if found_x == 0 {
            let state_x = State::from_moons(&moons, 0);
            if let Some(pos) = existing_states_x.get(&state_x) {
                println!("X loops at {}, loops to {}", step, pos);
                found_x = step;
            } else {
                existing_states_x.insert(state_x, step);
            }
        }

        if found_y == 0 {
            let state_y = State::from_moons(&moons, 1);
            if let Some(pos) = existing_states_y.get(&state_y) {
                println!("Y loops at {}, loops to {}", step, pos);
                found_y = step;
            } else {
                existing_states_y.insert(state_y, step);
            }
        }

        if found_z == 0 {
            let state_z = State::from_moons(&moons, 2);
            if let Some(pos) = existing_states_z.get(&state_z) {
                println!("Z loops at {}, loops to {}", step, pos);
                found_z = step;
            } else {
                existing_states_z.insert(state_z, step);
            }
        }
        if found_x != 0 && found_y != 0 && found_z != 0 {
            break;
        }
    }
    found_x.lcm(&found_y).lcm(&found_z)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let moons = [
        Moon::new([-6, -5, -8]),
        Moon::new([0, -3, -13]),
        Moon::new([-15, 10, -11]),
        Moon::new([-3, -8, 3]),
    ];

    let mut moons_part_1 = moons.clone();
    for _ in 0..1000 {
        simulation_step(&mut moons_part_1);
    }
    let result: i32 = moons_part_1.iter().map(|v| v.get_total_energy()).sum();
    println!("result part1: {}", result);

    let result = find_loop(moons);
    println!("result part2: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOONS: [Moon; 4] = [
        Moon::new([-1, 0, 2]),
        Moon::new([2, -10, -7]),
        Moon::new([4, -8, 8]),
        Moon::new([3, 5, -1]),
    ];

    #[test]
    fn test_apply_gravity() {
        let mut ganymede = Moon {
            pos: [3, 0, 0],
            velocity: [0, 0, 0],
        };
        let mut callisto = Moon {
            pos: [5, 0, 0],
            velocity: [0, 0, 0],
        };
        ganymede.apply_gravity(&mut callisto);
        assert_eq!(ganymede.velocity[0], 1);
        assert_eq!(callisto.velocity[0], -1);
    }

    #[test]
    fn test_simulation_step() {
        let mut moons = MOONS;
        simulation_step(&mut moons);
        assert_eq!(
            moons[0],
            Moon {
                pos: [2, -1, 1],
                velocity: [3, -1, -1]
            }
        );
        assert_eq!(
            moons[1],
            Moon {
                pos: [3, -7, -4],
                velocity: [1, 3, 3]
            }
        );
        assert_eq!(
            moons[2],
            Moon {
                pos: [1, -7, 5],
                velocity: [-3, 1, -3]
            }
        );
        assert_eq!(
            moons[3],
            Moon {
                pos: [2, 2, 0],
                velocity: [-1, -3, 1]
            }
        );
        for _ in 0..9 {
            simulation_step(&mut moons);
        }

        let total_energy: i32 = moons.iter().map(|v| v.get_total_energy()).sum();
        assert_eq!(179, total_energy);
    }

    #[test]
    fn test_find_loops() {
        let moons = MOONS;
        let result = find_loop(moons);
        assert_eq!(2772, result);
    }
}
