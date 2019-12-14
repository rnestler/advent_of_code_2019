use intcode_computer::{Machine, StepResult};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Tile {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
    Score(i64),
}

impl From<i64> for Tile {
    fn from(value: i64) -> Self {
        match value {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::HorizontalPaddle,
            4 => Tile::Ball,
            s => Tile::Score(s),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}
impl Pos {
    pub const fn new(x: i64, y: i64) -> Self {
        Pos { x, y }
    }
}
struct Arcade {
    pub machine: Machine,
    pub screen: HashMap<Pos, Tile>,
    pub ball: Pos,
    pub paddle: Pos,
    //pub walls: HashSet<Pos>,
    //pub blocks: HashSet<Pos>,
    //pub paddles: HashSet<Pos>,
    //pub balls: HashSet<Pos>,
}

struct ScoreCount {
    magic_pos_count: i64,
}

impl ScoreCount {
    pub fn new() -> ScoreCount {
        ScoreCount { magic_pos_count: 0 }
    }
    pub fn check_score(&mut self, pos: Pos, score: i64) -> Option<i64> {
        const MAGIC_POS: Pos = Pos::new(-1, 0);
        if pos == MAGIC_POS {
            self.magic_pos_count += 1;
        } else {
            self.magic_pos_count = 0;
        }
        if self.magic_pos_count == 3 {
            self.magic_pos_count = 0;
            Some(score)
        } else {
            None
        }
    }

    pub fn decode_output_chunk(&mut self, chunk: &[i64]) -> (Pos, Tile) {
        let pos = Pos::new(chunk[0], chunk[1]);
        if let Some(score) = self.check_score(pos, chunk[2]) {
            (pos, Tile::Score(score))
        } else {
            (pos, Tile::from(chunk[2]))
        }
    }
}

impl Arcade {
    pub fn new(machine: Machine) -> Arcade {
        Arcade {
            machine,
            screen: HashMap::new(),
            ball: Pos::new(0, 0),
            paddle: Pos::new(0, 0),
        }
    }

    pub fn decode_output_chunk(chunk: &[i64]) -> (Pos, Tile) {
        (Pos::new(chunk[0], chunk[1]), Tile::from(chunk[2]))
    }

    pub fn run_part_1(&mut self) -> usize {
        let _ = self.machine.run_until_block();
        let output = self.machine.drain_output();
        for chunk in output.chunks_exact(3) {
            let (pos, tile) = Self::decode_output_chunk(chunk);
            if tile == Tile::Block {
                self.screen.insert(pos, tile);
            }
        }
        self.screen.keys().count()
    }

    pub fn run_part_2(&mut self) -> i64 {
        let mut score = 0;
        let mut score_count = ScoreCount::new();
        self.machine.set_state(0, 2);
        loop {
            match self.machine.run_until_block() {
                StepResult::Halt(_) => {
                    let output = self.machine.drain_output();
                    for chunk in output.chunks_exact(3) {
                        let (_pos, tile) = score_count.decode_output_chunk(chunk);
                        match tile {
                            Tile::Score(s) => {
                                score = s;
                            }
                            _ => {}
                        }
                    }
                    return score;
                }
                StepResult::NeedsInput => {
                    let output = self.machine.drain_output();
                    for chunk in output.chunks_exact(3) {
                        let (pos, tile) = score_count.decode_output_chunk(chunk);
                        match tile {
                            Tile::Score(s) => {
                                score = s;
                            }
                            _ => {
                                self.screen.insert(pos, tile);
                            }
                        }
                        if tile == Tile::Ball {
                            self.ball = pos;
                        }
                        if tile == Tile::HorizontalPaddle {
                            self.paddle = pos;
                        }
                    }

                    if self.ball.x < self.paddle.x {
                        self.machine.add_input(-1);
                    } else if self.ball.x > self.paddle.x {
                        self.machine.add_input(1);
                    } else {
                        self.machine.add_input(0);
                    }

                    let blocks = self.screen.values().filter(|v| v == &&Tile::Block).count();
                }
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let code = Machine::read_code("input.txt")?;

    let mut arcade = Arcade::new(Machine::new(code.clone()));
    let result = arcade.run_part_1();
    println!("result part1: {}", result);

    let mut arcade = Arcade::new(Machine::new(code.clone()));
    let result = arcade.run_part_2();
    println!("result part2: {}", result);

    Ok(())
}
