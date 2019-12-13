use intcode_computer::Machine;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
}

impl From<i64> for Tile {
    fn from(value: i64) -> Self {
        match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            _ => panic!("unknown tile"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}
impl Pos {
    pub fn new(x: i64, y: i64) -> Self {
        Pos { x, y }
    }
}
struct Arcade {
    pub machine: Machine,
    pub screen: HashMap<Pos, Tile>,
    pub ball: Pos,
    //pub walls: HashSet<Pos>,
    //pub blocks: HashSet<Pos>,
    //pub paddles: HashSet<Pos>,
    //pub balls: HashSet<Pos>,
}

impl Arcade {
    pub fn new(machine: Machine) -> Arcade {
        Arcade {
            machine,
            screen: HashMap::new(),
            ball: Pos::new(0, 0),
        }
    }

    pub fn run_part_1(&mut self) -> usize {
        let _ = self.machine.run_until_block();
        let output = self.machine.drain_output();
        for chunk in output.chunks_exact(3) {
            let pos = Pos::new(chunk[0], chunk[1]);
            let tile = Tile::from(chunk[2]);
            if tile == Tile::Block {
                self.screen.insert(pos, tile);
            }
        }
        self.screen.keys().count()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = Machine::read_code("input.txt")?;
    let mut arcade = Arcade::new(Machine::new(code));

    let result = arcade.run_part_1();
    println!("result part1: {}", result);

    Ok(())
}
