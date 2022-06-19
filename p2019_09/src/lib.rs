use std::collections::BTreeMap;
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

    fn exec_one(&mut self) {
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
            self.exec_one();
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let program: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let mut computer = IntComputer::new(&program);
    computer.input.push_back(1);
    computer.exec();
    computer.output.pop_front().unwrap()
}

pub fn part_b(input: &str) -> i64 {
    let program: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let mut computer = IntComputer::new(&program);
    computer.input.push_back(2);
    computer.exec();
    computer.output.pop_front().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 3601950151);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 64236);
    }
}
