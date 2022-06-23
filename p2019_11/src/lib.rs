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

pub fn part_a(input: &str) -> usize {
    let program: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let mut computer = IntComputer::new(&program);
    let mut map = HashMap::new();
    let mut loc_x = 0;
    let mut loc_y = 0;
    map.insert((loc_x, loc_y), 0);
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut idx: i32 = 2;
    loop {
        let in1 = map.get(&(loc_x, loc_y)).unwrap_or(&0);
        computer.input.push_back(*in1);
        computer.exec_one();
        computer.exec_one();
        if computer.finished() {
            break;
        }
        let o1 = computer.output.pop_front().unwrap();
        let o2 = computer.output.pop_front().unwrap();
        *map.entry((loc_x, loc_y)).or_insert(0) = o1;
        match o2 {
            // turn left
            0 => idx += 1,
            // turn right
            1 => idx -= 1,
            _ => panic!("invalid direction: {}", o2),
        }
        idx = (idx + directions.len() as i32) % directions.len() as i32;
        loc_x += directions[idx as usize].0;
        loc_y += directions[idx as usize].1;
    }
    map.iter().count()
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
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            print!("{}", map.get(&(x, y)).unwrap_or(&' '));
        }
        println!();
    }
}

pub fn part_b(input: &str) {
    let program: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let mut computer = IntComputer::new(&program);
    let mut map = HashMap::new();
    let mut loc_x = 0;
    let mut loc_y = 0;
    map.insert((loc_x, loc_y), 1);
    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut idx: i32 = 2;
    loop {
        let in1 = map.get(&(loc_x, loc_y)).unwrap_or(&0);
        computer.input.push_back(*in1);
        computer.exec_one();
        computer.exec_one();
        if computer.finished() {
            break;
        }
        let o1 = computer.output.pop_front().unwrap();
        let o2 = computer.output.pop_front().unwrap();
        *map.entry((loc_x, loc_y)).or_insert(0) = o1;
        match o2 {
            // turn left
            0 => idx += 1,
            // turn right
            1 => idx -= 1,
            _ => panic!("invalid direction: {}", o2),
        }
        idx = (idx + directions.len() as i32) % directions.len() as i32;
        loc_x += directions[idx as usize].0;
        loc_y += directions[idx as usize].1;
    }

    let mut final_map = HashMap::new();
    for (k, v) in map {
        match v {
            0 => final_map.insert(k, ' '),
            1 => final_map.insert(k, '#'),
            _ => panic!("invalid color: {}", v),
        };
    }
    print_map(&final_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 1883);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        part_b(input);
    }
}
