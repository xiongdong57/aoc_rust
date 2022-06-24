use core::panic;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
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

pub fn part_a(input: &str) -> i64 {
    let program: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let computer = IntComputer::new(&program);
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    q.push_back((0, 0, 0, computer));
    // north (1), south (2), west (3), and east (4)
    let directions = [(1, (0, 1)), (2, (0, -1)), (3, (-1, 0)), (4, (1, 0))];
    let mut steps_to_found = 0;
    loop {
        if q.is_empty() {
            break;
        }
        let (x, y, steps, computer) = q.pop_front().unwrap();
        seen.insert((x, y));
        for (ins, (dx, dy)) in directions {
            let mut cur_computer = computer.clone();
            cur_computer.input.push_back(ins);
            cur_computer.exec_once();
            if cur_computer.finished() {
                continue;
            }
            let out = cur_computer.output.pop_front().unwrap();
            match out {
                // The repair droid hit a wall. Its position has not changed
                0 => (),
                // The repair droid has moved one step in the requested direction.
                1 => {
                    if !seen.contains(&(x + dx, y + dy)) {
                        q.push_front((x + dx, y + dy, steps + 1, cur_computer));
                    }
                }
                // The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.
                2 => {
                    if !seen.contains(&(x + dx, y + dy)) {
                        q.push_front((x + dx, y + dy, steps + 1, cur_computer));
                        steps_to_found = steps + 1;
                    }
                }
                _ => panic!("invalid output {}", out),
            }
        }
    }
    steps_to_found
}

pub fn part_b(input: &str) -> i64 {
    let program: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let computer = IntComputer::new(&program);
    let mut q = VecDeque::new();
    let mut map = HashSet::new();
    q.push_back((0, 0, 0, computer));
    // north (1), south (2), west (3), and east (4)
    let directions = [(1, (0, 1)), (2, (0, -1)), (3, (-1, 0)), (4, (1, 0))];
    let mut loc_x = 0;
    let mut loc_y = 0;
    loop {
        if q.is_empty() {
            break;
        }
        let (x, y, steps, computer) = q.pop_front().unwrap();
        map.insert((x, y));
        for (ins, (dx, dy)) in directions {
            let mut cur_computer = computer.clone();
            cur_computer.input.push_back(ins);
            cur_computer.exec_once();
            if cur_computer.finished() {
                continue;
            }
            let out = cur_computer.output.pop_front().unwrap();
            match out {
                // The repair droid hit a wall. Its position has not changed
                0 => (),
                // The repair droid has moved one step in the requested direction.
                1 => {
                    if !map.contains(&(x + dx, y + dy)) {
                        q.push_front((x + dx, y + dy, steps + 1, cur_computer));
                    }
                }
                // The repair droid has moved one step in the requested direction; its new position is the location of the oxygen system.
                2 => {
                    if !map.contains(&(x + dx, y + dy)) {
                        q.push_front((x + dx, y + dy, steps + 1, cur_computer));
                        loc_x = x + dx;
                        loc_y = y + dy;
                    }
                }
                _ => panic!("invalid output {}", out),
            }
        }
    }
    let mut filled = Vec::new();
    filled.push((loc_x, loc_y));
    for minutes in 1.. {
        // filled every nearby dot in map
        for i in 0..filled.len() {
            let (x, y) = filled[i];
            for (_, (dx, dy)) in directions {
                let new_x = x + dx;
                let new_y = y + dy;
                if map.contains(&(new_x, new_y)) && (!filled.contains(&(new_x, new_y))) {
                    filled.push((new_x, new_y));
                }
            }
            if filled.len() == map.len() {
                return minutes;
            }
        }
    }
    panic!("no solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 218);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 544);
    }
}
