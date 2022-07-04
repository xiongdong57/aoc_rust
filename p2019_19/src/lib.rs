use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct IntComputer {
    mem: HashMap<usize, i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    curr: usize,
    exit: bool,
    relative_base: i64,
}

#[allow(dead_code)]
impl IntComputer {
    fn new(progrm: &Vec<i64>) -> Self {
        let mut mem = HashMap::new();
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

pub fn part_a(input: &str) -> i64 {
    let program: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let computer = IntComputer::new(&program);
    let mut map = HashMap::new();
    let mut cache: HashMap<(i64, i64), i64> = HashMap::new();
    for x in 0..50 {
        for y in 0..50 {
            let out = get_res(&computer, x, y, &mut cache);
            match out {
                0 => map.insert((x, y), '.'),
                1 => map.insert((x, y), '#'),
                _ => panic!("invalid output"),
            };
        }
    }
    map.iter().filter(|(_, v)| **v == '#').count() as i64
}

fn get_res(computer: &IntComputer, x: i64, y: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if let Some(out) = cache.get(&(x, y)) {
        return *out;
    }
    let mut cur_computer = computer.clone();
    cur_computer.input.push_back(x);
    cur_computer.input.push_back(y);
    cur_computer.exec_once();
    let out = cur_computer.output.pop_front().unwrap();
    cache.insert((x, y), out);
    out
}

pub fn part_b(input: &str) -> i64 {
    let program: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let computer = IntComputer::new(&program);
    let mut cache: HashMap<(i64, i64), i64> = HashMap::new();
    let width = 99;
    let mut left = 0;
    let mut new_line: bool = false;
    for i in 1500.. {
        for j in left.. {
            // 1523 1022,
            // if i start from 0, it costs 207s to run
            // if i start from 1000, it costs 127s to run
            // if i start from 1500, it costs 10s to run
            // maybe improve it later.
            if get_res(&computer, i, j, &mut cache) == 1 {
                if get_res(&computer, i + width, j + width, &mut cache) == 1
                    && get_res(&computer, i, j + width, &mut cache) == 1
                    && get_res(&computer, i + width, j, &mut cache) == 1
                {
                    return i * 10000 + j;
                }
                if new_line == false {
                    left = j;
                    new_line = true;
                }
            } else {
                if new_line == true {
                    new_line = false;
                    break;
                }
            }
            if j > i {
                new_line = false;
                break;
            }
        }
    }

    panic!("No solution");
}

#[allow(dead_code)]
fn print_map(map: &HashMap<(i64, i64), char>) {
    let (min_x, min_y) = map
        .keys()
        .fold((i64::MAX, i64::MAX), |(min_x, min_y), (x, y)| {
            (min_x.min(*x), min_y.min(*y))
        });
    let (max_x, max_y) = map
        .keys()
        .fold((i64::MIN, i64::MIN), |(max_x, max_y), (x, y)| {
            (max_x.max(*x), max_y.max(*y))
        });
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            print!("{}", map.get(&(x, y)).unwrap());
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 131);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 15231022);
    }
}
