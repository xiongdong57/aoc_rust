use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct IntComputer {
    mem: BTreeMap<usize, i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    curr: usize,
    exit: bool,
    relative_base: i64,
}

#[allow(dead_code)]
impl IntComputer {
    fn new(progrm: &Vec<i64>) -> Self {
        let mut mem = BTreeMap::new();
        for (i, x) in progrm.iter().enumerate() {
            mem.insert(i as usize, *x);
        }
        IntComputer {
            mem: mem,
            input: VecDeque::new(),
            output: VecDeque::new(),
            curr: 0,
            exit: false,
            relative_base: 0,
        }
    }

    // return the value of the pos of instruction
    fn resolve(&self, mode: i64, curr: usize) -> i64 {
        match mode {
            // position mode
            0 => {
                let p = self.mem[&curr] as usize;
                self.mem.get(&p).unwrap_or(&0).to_owned()
            }
            // immediate mode
            1 => self.mem[&curr],
            // relative mode
            2 => {
                let p = (self.mem[&curr] + self.relative_base) as usize;
                self.mem.get(&p).unwrap_or(&0).to_owned()
            }
            _ => panic!("invalid mode"),
        }
    }

    // return the index of the pos of instruction
    fn resolve_idx(&self, mode: i64, curr: usize) -> usize {
        match mode {
            // position mode
            0 => self.mem[&curr] as usize,
            // relative mode
            2 => (self.mem[&curr] + self.relative_base) as usize,
            _ => panic!("invalid mode"),
        }
    }

    fn exec_once(&mut self) {
        if self.exit {
            return;
        }
        let mut curr = self.curr;
        loop {
            let next = format!("{:0>5}", self.mem[&(curr as usize)]);
            // eg: 01002
            let opcode = next[3..5].parse::<i64>().unwrap();
            let mode1 = next[2..3].parse::<i64>().unwrap();
            let mode2 = next[1..2].parse::<i64>().unwrap();
            let mode3 = next[0..1].parse::<i64>().unwrap();
            match opcode {
                1 => {
                    let v1 = self.resolve(mode1, curr + 1);
                    let v2 = self.resolve(mode2, curr + 2);
                    let p3 = self.resolve_idx(mode3, curr + 3);
                    *self.mem.entry(p3).or_insert(0) = v1 + v2;
                    curr += 4;
                }
                2 => {
                    let v1 = self.resolve(mode1, curr + 1);
                    let v2 = self.resolve(mode2, curr + 2);
                    let p3 = self.resolve_idx(mode3, curr + 3);
                    *self.mem.entry(p3).or_insert(0) = v1 * v2;
                    curr += 4;
                }
                3 => {
                    let p1 = self.resolve_idx(mode1, curr + 1);
                    *self.mem.entry(p1).or_insert(0) = self.input.pop_front().unwrap();
                    curr += 2;
                }
                4 => {
                    self.output.push_back(self.resolve(mode1, curr + 1));
                    curr += 2;
                    break;
                }
                5 => {
                    let v1 = self.resolve(mode1, curr + 1);
                    let v2 = self.resolve(mode2, curr + 2);
                    match v1 {
                        i if i != 0 => curr = v2 as usize,
                        _ => curr += 3,
                    }
                }
                6 => {
                    let v1 = self.resolve(mode1, curr + 1);
                    let v2 = self.resolve(mode2, curr + 2);
                    match v1 {
                        0 => curr = v2 as usize,
                        _ => curr += 3,
                    }
                }
                7 => {
                    let v1 = self.resolve(mode1, curr + 1);
                    let v2 = self.resolve(mode2, curr + 2);
                    let p3 = self.resolve_idx(mode3, curr + 3);
                    *self.mem.entry(p3).or_insert(0) = match v1 {
                        i if i < v2 => 1,
                        _ => 0,
                    };
                    curr += 4;
                }
                8 => {
                    let v1 = self.resolve(mode1, curr + 1);
                    let v2 = self.resolve(mode2, curr + 2);
                    let p3 = self.resolve_idx(mode3, curr + 3);
                    *self.mem.entry(p3).or_insert(0) = match v1 {
                        i if i == v2 => 1,
                        _ => 0,
                    };

                    curr += 4;
                }
                9 => {
                    let v1 = self.resolve(mode1, curr + 1);
                    self.relative_base += v1;
                    curr += 2;
                }
                99 => {
                    self.exit = true;
                    break;
                }
                _ => panic!("Invalid opcode: {}", next),
            }
        }
        self.curr = curr;
    }

    fn finished(&self) -> bool {
        self.exit
    }

    fn exec(&mut self) {
        while !self.finished() {
            self.exec_once();
        }
    }
}

pub fn part_a(input: &str) -> usize {
    let program: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let mut computer = IntComputer::new(&program);
    let mut map: HashMap<(i64, i64), Tile> = HashMap::new();
    loop {
        computer.exec_once();
        computer.exec_once();
        computer.exec_once();
        if computer.finished() {
            break;
        }
        let x = computer.output.pop_front().unwrap();
        let y = computer.output.pop_front().unwrap();
        let o3 = computer.output.pop_front().unwrap();
        let tile = match o3 {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("no such tile num: {}", o3),
        };
        map.insert((x, y), tile);
        // ball will break block
    }
    map.values().filter(|v| **v == Tile::Block).count()
}

#[derive(PartialEq, Eq, Debug)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

pub fn part_b(input: &str) -> i64 {
    let program: Vec<i64> = format!("{}", input)
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let mut computer = IntComputer::new(&program);
    // computer.mem[&0] = 2;
    computer.mem.insert(0, 2);
    let mut map: HashMap<(i64, i64), Tile> = HashMap::new();
    let mut score = 0;
    loop {
        computer.exec_once();
        computer.exec_once();
        computer.exec_once();
        if computer.finished() {
            break;
        }
        let x = computer.output.pop_front().unwrap();
        let y = computer.output.pop_front().unwrap();
        let o3 = computer.output.pop_front().unwrap();
        if x == -1 && y == 0 {
            score = o3;
        } else {
            let tile = match o3 {
                0 => Tile::Empty,
                1 => Tile::Wall,
                2 => Tile::Block,
                3 => Tile::Paddle,
                4 => Tile::Ball,
                _ => panic!("no such tile num: {}", o3),
            };
            map.insert((x, y), tile);
        }
        // ball will break block
        // find ball and Paddle
        let ball = map.iter().filter(|v| *v.1 == Tile::Ball).next();
        let paddle = map.iter().filter(|v| *v.1 == Tile::Paddle).next();
        if ball.is_some() && paddle.is_some() {
            let ball_x = ball.unwrap().0 .0;
            let paddle_x = paddle.unwrap().0 .0;
            if paddle_x == ball_x {
                computer.input.push_front(0);
            } else if paddle_x > ball_x {
                computer.input.push_front(-1);
            } else {
                computer.input.push_front(1);
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 326);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 15988);
    }
}
