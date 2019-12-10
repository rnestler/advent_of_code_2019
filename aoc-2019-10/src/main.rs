use std::fs::read;
use std::str;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct AngleDistance {
    angle: f64,
    distance: f64,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    // if the normalized difference is the same, the points lay on a line
    pub fn norm_diff(&self, other: &Self) -> (i32, i32) {
        let diff = Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        };

        // we round to 3 decimal places to avoid precision issues
        let len = ((diff.x * diff.x + diff.y * diff.y) as f64).sqrt();
        (
            (diff.x as f64 / len * 1000.).round() as i32,
            (diff.y as f64 / len * 1000.).round() as i32,
        )
    }

    pub fn get_angle_distance(&self, other: Self) -> AngleDistance {
        let diff = (
            self.x as f64 - other.x as f64,
            self.y as f64 - other.y as f64,
        );

        let mut angle = diff.1.atan2(diff.0);
        // move last quadrant (-pi3/4..-pi/2) at the end
        if angle < -std::f64::consts::PI / 2.0 {
            angle += 2.0 * std::f64::consts::PI;
        }
        let distance = (diff.0 * diff.0 + diff.1 * diff.1).sqrt();
        AngleDistance { angle, distance }
    }
}

fn get_visible_asteroids(asteroids: &Vec<Pos>) -> Vec<usize> {
    let mut result = vec![];
    for i in 0..asteroids.len() {
        let mut norms = vec![];
        for j in 0..asteroids.len() {
            if j == i {
                continue;
            }

            let norm_diff = asteroids[i].norm_diff(&asteroids[j]);
            if !norms.contains(&norm_diff) {
                norms.push(norm_diff);
            }
        }
        let visible = norms.len();
        result.push(visible);
    }
    result
}

fn get_asteroids(input: &str) -> Vec<Pos> {
    let mut asteroids = vec![];
    let mut row = 0;
    for line in input.lines() {
        let mut col = 0;
        for b in line.bytes() {
            match b {
                b'#' => asteroids.push(Pos::new(col, row)),
                b'.' => {}
                _ => panic!("illegal character"),
            }
            col += 1;
        }
        row += 1;
    }
    asteroids
}

fn get_200th_vaporized(asteroids: Vec<Pos>, battle_station: Pos) -> Pos {
    #[derive(Debug, Clone)]
    struct Asteroid {
        pos: Pos,
        angle_distance: AngleDistance,
    }
    let mut asteroids: Vec<_> = asteroids
        .iter()
        .map(|a| Asteroid {
            pos: *a,
            angle_distance: a.get_angle_distance(battle_station),
        })
        .collect();
    asteroids.sort_by(|a, b| {
        a.angle_distance
            .distance
            .partial_cmp(&b.angle_distance.distance)
            .unwrap()
    });
    asteroids.sort_by(|a, b| {
        a.angle_distance
            .angle
            .partial_cmp(&b.angle_distance.angle)
            .unwrap()
    });

    loop {
        let mut n_vaporized = 0;
        let mut old_angle = -10.0;
        let asteroids_copy = asteroids.clone();
        let mut to_remove = vec![];
        for (i, a) in asteroids_copy.iter().enumerate() {
            if a.angle_distance.angle == old_angle {
                continue;
            }
            old_angle = a.angle_distance.angle;
            n_vaporized += 1;
            if n_vaporized == 200 {
                return a.pos;
            }
            to_remove.push(i);
        }
        for i in &to_remove {
            asteroids.remove(*i);
        }
        if asteroids.is_empty() {
            panic!("should not happen");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = read("input.txt")?;
    let asteroids = get_asteroids(&str::from_utf8(&content)?);
    let visible = get_visible_asteroids(&asteroids);
    let (pos, max) = visible.iter().enumerate().max_by_key(|v| v.1).unwrap();
    println!("Result {:?}", max);

    let battle_station = asteroids[pos];
    let mut asteroids = asteroids;
    asteroids.remove(pos);

    let asteroid = get_200th_vaporized(asteroids, battle_station);
    println!("Result part 2: {}", asteroid.x * 100 + asteroid.y);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = ".#..#
.....
#####
....#
...##
";

    const INPUT_BIG: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    #[test]
    fn test_get_asteroids() {
        let input = INPUT_1;
        let asteroids = get_asteroids(input);
        assert_eq!(
            asteroids,
            vec![
                Pos { x: 1, y: 0 },
                Pos { x: 4, y: 0 },
                Pos { x: 0, y: 2 },
                Pos { x: 1, y: 2 },
                Pos { x: 2, y: 2 },
                Pos { x: 3, y: 2 },
                Pos { x: 4, y: 2 },
                Pos { x: 4, y: 3 },
                Pos { x: 3, y: 4 },
                Pos { x: 4, y: 4 }
            ]
        );
    }

    #[test]
    fn test_get_visible_asteroids() {
        let input = INPUT_1;
        let asteroids = get_asteroids(input);
        let visible_asteroids = get_visible_asteroids(&asteroids);

        assert_eq!(visible_asteroids, vec![7, 7, 6, 7, 7, 7, 5, 7, 8, 7]);
    }

    #[test]
    fn test_get_visible_max_1() {
        let input = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
        let asteroids = get_asteroids(input);
        let visible_asteroids = get_visible_asteroids(&asteroids);
        assert_eq!(visible_asteroids.iter().max(), Some(&33));
    }

    #[test]
    fn test_get_visible_max_2() {
        let input = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
        let asteroids = get_asteroids(input);
        let visible_asteroids = get_visible_asteroids(&asteroids);
        assert_eq!(visible_asteroids.iter().max(), Some(&35));
    }

    #[test]
    fn test_get_visible_max_3() {
        let input = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
        let asteroids = get_asteroids(input);
        let visible_asteroids = get_visible_asteroids(&asteroids);
        assert_eq!(visible_asteroids.iter().max(), Some(&41));
    }

    #[test]
    fn test_get_visible_max_4() {
        let input = INPUT_BIG;
        let asteroids = get_asteroids(input);
        let visible_asteroids = get_visible_asteroids(&asteroids);
        assert_eq!(visible_asteroids.iter().max(), Some(&210));
    }

    #[test]
    fn test_200th_vaporized() {
        let input = INPUT_BIG;
        let asteroids = get_asteroids(input);
        let visible = get_visible_asteroids(&asteroids);
        let (pos, _) = visible.iter().enumerate().max_by_key(|v| v.1).unwrap();
        let battle_station = asteroids[pos];

        let mut asteroids = asteroids;
        asteroids.remove(pos);

        let asteroid = get_200th_vaporized(asteroids, battle_station);
        assert_eq!(asteroid, Pos::new(8, 2));
    }
}
