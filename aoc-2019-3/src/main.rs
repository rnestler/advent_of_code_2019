use geo::Line;
use line_intersection::LineInterval;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy)]
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

//impl Sub for Point {}

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

#[derive(Debug, Clone)]
pub struct Path {
    pub points: Vec<Point>,
}

impl Path {
    fn new() -> Self {
        Path {
            points: vec![Point { x: 0, y: 0 }],
        }
    }

    fn line_intersection(line1: (&Point, &Point), line2: (&Point, &Point)) -> Option<Point> {
        let segment1 = LineInterval::line_segment(Line {
            start: (line1.0.x as f64, line1.0.y as f64).into(),
            end: (line1.1.x as f64, line1.1.y as f64).into(),
        });

        let segment2 = LineInterval::line_segment(Line {
            start: (line2.0.x as f64, line2.0.y as f64).into(),
            end: (line2.1.x as f64, line2.1.y as f64).into(),
        });

        let intersection = segment1.relate(&segment2).unique_intersection();
        intersection.map(|point| Point {
            x: point.x() as i32,
            y: point.y() as i32,
        })
    }

    pub fn find_closest_intersection_distance(&self, other: &Self) -> i32 {
        let mut closest_distance = std::i32::MAX;
        for line1 in self.points[1..].iter().zip(self.points[2..].iter()) {
            for line2 in other.points[1..].iter().zip(other.points[2..].iter()) {
                if let Some(point) = Self::line_intersection(line1, line2) {
                    let distance = point.distance_to_central_port();
                    println!("Found intersection: {:?}: {}", point, distance);
                    closest_distance = i32::min(closest_distance, distance);
                }
            }
        }
        closest_distance
    }
}

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        let mut path = Path::new();
        for s in s.trim().split(',') {
            let segment = Segment::from(s);
            print!("{:?}, ", segment);
            let point = path.points.last().unwrap().add_segment(&segment);
            path.points.push(point);
        }
        println!();
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

    println!("{:?}", paths);

    let result = paths[0].find_closest_intersection_distance(&paths[1]);
    println!("result: {}", result);

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

    #[test]
    fn test_find_closest_intersection_distance_2() {
        let path1 = Path::from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let path2 = Path::from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(path1.find_closest_intersection_distance(&path2), 135);
    }
}
