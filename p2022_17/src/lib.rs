use std::{collections::HashSet, ops::Add};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Jet {
    Left,
    Right,
}

impl TryFrom<char> for Jet {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

macro_rules! point {
    ($x:expr, $y:expr) => {
        Point { x: $x, y: $y }
    };
}

impl Add for Point {
    type Output = Self;
    fn add(self, r: Self) -> Self::Output {
        Self {
            x: self.x + r.x,
            y: self.y + r.y,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Rock {
    Minus,
    Plus,
    L,
    Pipe,
    Square,
}

impl Rock {
    fn points(&self) -> HashSet<Point> {
        match self {
            Self::Minus => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Self::Plus => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Self::L => vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
            Self::Pipe => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Self::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
        .iter()
        .map(|(x, y)| Point {
            x: *x as i64,
            y: *y as i64,
        })
        .collect::<HashSet<Point>>()
    }
    fn index(index: usize) -> Self {
        match index % 5 {
            0 => Self::Minus,
            1 => Self::Plus,
            2 => Self::L,
            3 => Self::Pipe,
            4 => Self::Square,
            _ => unreachable!(),
        }
    }
}

const WIDTH: i64 = 7;
const MAX_X: i64 = WIDTH - 1;

struct Simulation {
    /// The number of rocks dropped so far.
    dropped: usize,
    /// The jets.
    jets: Vec<Jet>,
    /// The current position of the rock being dropped.
    pos: Point,
    /// Solidified rocks.
    solid: HashSet<Point>,
    /// The height of the pile.
    height: i64,
    /// The number of turns since the start of the simulation.
    turn: usize,
}

impl Simulation {
    fn new(jets: Vec<Jet>) -> Self {
        Self {
            dropped: 0,
            jets,
            pos: point!(2, 3),
            solid: (0..WIDTH).map(|x| point!(x, -1)).collect(),
            height: 0,
            turn: 0,
        }
    }

    fn free(&self, rock: Rock, offset: Point) -> bool {
        rock.points().iter().any(|&p| {
            let p = p + self.pos + offset;

            self.solid.contains(&p) || p.y < 0 || p.x < 0 || p.x > MAX_X
        })
    }
    fn drop(&mut self) -> i64 {
        loop {
            let jet = self.turn % self.jets.len();
            let rock = Rock::index(self.dropped);

            match self.jets.get(jet) {
                Some(Jet::Left) if !self.free(rock, point!(-1, 0)) => {
                    self.pos.x -= 1;
                }
                Some(Jet::Right) if !self.free(rock, point!(1, 0)) => {
                    self.pos.x += 1;
                }
                _ => {}
            };

            self.turn += 1;

            if self.free(rock, point!(0, -1)) {
                self.dropped += 1;

                for p in rock.points() {
                    let p = p + self.pos;

                    self.height = self.height.max(p.y + 1);
                    self.solid.insert(p);
                }

                self.pos = point!(2, self.height + 3);

                return self.height;
            }

            self.pos.y -= 1;
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    // https://github.com/bsdrks/aoc2022/blob/main/src/bin/day17a.rs
    let mut simulation = Simulation::new(
        input
            .chars()
            .map(|c| Jet::try_from(c).map_or_else(|_| panic!(), |direction| direction))
            .collect::<Vec<Jet>>(),
    );

    // MAX_ROCKS: usize = 2022;
    while simulation.dropped < 2022 {
        simulation.drop();
    }

    simulation.height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 3068);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 3085);
    }
}
