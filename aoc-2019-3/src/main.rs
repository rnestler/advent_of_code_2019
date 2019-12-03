use std::fs::File;
use std::io::prelude::*;

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn distance(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn distance_to_central_port(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn add_segment(&self, segment: &Segment) -> Self {
        match segment {
            Segment::Up(value) => Point {
                x: self.x,
                y: self.y + value,
            },
            Segment::Down(value) => Point {
                x: self.x,
                y: self.y - value,
            },
            Segment::Left(value) => Point {
                x: self.x - value,
                y: self.y,
            },
            Segment::Right(value) => Point {
                x: self.x + value,
                y: self.y,
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Segment {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl From<&str> for Segment {
    fn from(s: &str) -> Self {
        let value = i32::from_str_radix(&s[1..], 10).expect("parse error");
        match s.as_bytes()[0] {
            b'U' => Segment::Up(value),
            b'D' => Segment::Down(value),
            b'L' => Segment::Left(value),
            b'R' => Segment::Right(value),
            _ => {
                panic!("Unexpected token");
            }
        }
    }
}

pub struct Path {
    pub points: Vec<Point>,
}

impl Path {
    fn new() -> Self {
        Path {
            points: vec![Point { x: 0, y: 0 }],
        }
    }

    pub fn find_closest_intersection_distance(&self, other: &Self) -> i32 {
        0
    }
}

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        let mut path = Path::new();
        for s in s.trim().split(',') {
            let segment = Segment::from(s);
            let point = path.points.last().unwrap().add_segment(&segment);
            path.points.push(point);
        }
        path
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let paths = contents
        .lines()
        .map(|line| Path::from(line))
        .collect::<Vec<_>>();

    for path in &paths {
        println!("len: {}", path.points.len());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segment_from_slice() {
        let segment = Segment::from("U123");
        assert_eq!(segment, Segment::Up(123));
    }

    #[test]
    fn test_find_closest_intersection_distance() {
        let path1 = Path::from("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let path2 = Path::from("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(path1.find_closest_intersection_distance(&path2), 159);
    }
}
