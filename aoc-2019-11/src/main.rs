use intcode_computer::{Machine, StepResult};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_left(&mut self) {
        use Direction::*;
        *self = match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    pub fn turn_right(&mut self) {
        use Direction::*;
        *self = match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Color {
    Black = 0,
    White = 1,
}

struct Robot {
    machine: Machine,
    direction: Direction,
    position: Pos,
    panel: HashMap<Pos, Color>,
    default_color: Color,
}

impl Robot {
    pub fn new(machine: Machine, default_color: Color) -> Robot {
        Robot {
            machine,
            direction: Direction::Up,
            position: Pos::new(0, 0),
            panel: HashMap::new(),
            default_color,
        }
    }

    pub fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => {
                self.position.y -= 1;
            }
            Direction::Down => {
                self.position.y += 1;
            }
            Direction::Left => {
                self.position.x -= 1;
            }
            Direction::Right => {
                self.position.x += 1;
            }
        }
    }

    pub fn step(&mut self) -> StepResult {
        let current_color = self
            .panel
            .get(&self.position)
            .unwrap_or(&self.default_color);
        self.machine.add_input(*current_color as i64);
        let step_result = self.machine.run_until_block();
        let commands = self.machine.drain_output();
        if !commands.is_empty() {
            if commands[0] == 0 {
                self.panel.insert(self.position, Color::Black);
            } else {
                self.panel.insert(self.position, Color::White);
            }
            if commands[1] == 0 {
                self.direction.turn_left()
            } else {
                self.direction.turn_right()
            }
            self.move_forward();
        }
        step_result
    }

    pub fn run_part_1(&mut self) -> usize {
        loop {
            match self.step() {
                StepResult::Halt(_) => return self.panel.keys().count(),
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = Machine::read_code("input.txt")?;
    let machine = Machine::new(code.clone());

    let mut robot = Robot::new(machine, Color::Black);

    let result = robot.run_part_1();
    println!("result part 1: {}", result);

    let machine = Machine::new(code.clone());

    let mut robot = Robot::new(machine, Color::White);
    println!("{:?}", robot.panel);

    let result = robot.run_part_1();
    for y in -10..10 {
        for x in -50..50 {
            let color = robot.panel.get(&Pos::new(x, y)).unwrap_or(&Color::White);
            match color {
                Color::White => print!("#"),
                Color::Black => print!(" "),
            }
        }
        println!();
    }
    println!("result part 2: {}", result);

    Ok(())
}
