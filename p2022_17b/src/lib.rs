use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
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

/// A position in two-dimensional space.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    /// Left to right.
    x: i64,
    /// Depth.
    y: i64,
}

macro_rules! point {
    ($x:expr, $y:expr) => {
        Point { x: $x, y: $y }
    };
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

/// Rocks.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Rock {
    Minus,
    Plus,
    L,
    Pipe,
    Square,
}

impl Rock {
    #[must_use]
    pub fn points(&self) -> HashSet<Point> {
        match self {
            Self::Minus => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Self::Plus => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
            Self::L => vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
            Self::Pipe => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Self::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
        .iter()
        .map(|(x, y)| point!(*x, *y))
        .collect()
    }

    #[must_use]
    pub fn index(index: i64) -> Self {
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

const MAX_ROCKS: i64 = 1_000_000_000_000;
const WIDTH: i64 = 7;
const MAX_X: i64 = WIDTH - 1;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Key {
    tail: Vec<u8>,
    rock: Rock,
    jet: usize,
}

/// A simulation of rocks falling into a pit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Simulation {
    cache: HashMap<Key, (i64, i64)>,
    /// The number of rocks dropped so far.
    dropped: i64,
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
    #[must_use]
    pub fn new(jets: Vec<Jet>) -> Self {
        Self {
            cache: HashMap::new(),
            dropped: 0,
            jets,
            pos: point!(2, 3),
            solid: (0..WIDTH).map(|x| point!(x, -1)).collect(),
            height: 0,
            turn: 0,
        }
    }

    /// Get the top 100 rows as a vector, where each row is a byte.
    fn top(&self) -> Vec<u8> {
        (self.height - 100..self.height)
            .rev()
            .map(|y| {
                (0..WIDTH).fold(0, |mut byte, x| {
                    if self.solid.contains(&point!(x, y)) {
                        byte |= 1 << x;
                    }

                    byte
                })
            })
            .collect::<Vec<_>>()
    }

    fn free(&self, rock: Rock, offset: Point) -> bool {
        rock.points().iter().any(|&p| {
            let p = p + self.pos + offset;

            self.solid.contains(&p) || p.y < 0 || p.x < 0 || p.x > MAX_X
        })
    }

    /// Drop a rock. If a cycle is detected, return the length of the cycle and
    /// the height of the rocks in the cycle.
    pub fn drop(&mut self) -> Option<(i64, i64)> {
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
                if let Some(prev) = self.cache.insert(
                    Key {
                        rock,
                        jet,
                        tail: self.top(),
                    },
                    (self.height, self.dropped),
                ) {
                    let cycle = self.dropped - prev.1;
                    let height = self.height - prev.0;

                    return Some((cycle, height));
                }

                self.dropped += 1;

                for p in rock.points() {
                    let p = p + self.pos;

                    self.height = self.height.max(p.y + 1);
                    self.solid.insert(p);
                }

                self.pos = point!(2, self.height + 3);

                return None;
            }

            self.pos.y -= 1;
        }
    }
}

fn solve(input: &str) -> i64 {
    let mut simulation = Simulation::new(
        input
            .chars()
            .map(|c| Jet::try_from(c).map_or_else(|_| panic!(), |direction| direction))
            .collect::<Vec<Jet>>(),
    );

    loop {
        if let Some((cycle, height)) = simulation.drop() {
            let rocks = MAX_ROCKS - simulation.dropped;
            let cycles = rocks / cycle;

            // Drop the rocks that are not part of a cycle.
            for _ in 0..rocks % cycle {
                simulation.drop();
            }

            // `simulation.top` is the height of the rocks before any cycles.
            // `cycles * height` is the height of all rocks in a cycle.
            return simulation.height + cycles * height;
        }
    }
}

pub fn part_b(input: &str) -> i64 {
    solve(input.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 1514285714288);
    }
    // `1514285714289` not the right answer, will fix later

    // #[test]
    // fn part_b_result() {
    //     let input = include_str!("input.txt");
    //     assert_eq!(part_b(input), 0);
    // }
}
