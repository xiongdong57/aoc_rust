use std::collections::BTreeMap;
use std::collections::HashMap;
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
    let mut computer = IntComputer::new(&program);
    let mut x = 0;
    let mut y = 0;
    let mut map = HashMap::new();
    loop {
        computer.exec_once();
        if computer.finished() {
            break;
        }
        let out = computer.output.pop_front().unwrap();
        match out {
            35 => {
                map.insert((x, y), '#');
                x += 1;
            }
            46 => {
                map.insert((x, y), '.');
                x += 1;
            }
            10 => {
                x = 0;
                y -= 1;
            }
            _ => {
                map.insert((x, y), '^');
                x += 1;
            }
        };
    }
    // print_map(&map);
    let mut sum = 0;
    let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (x, y) in map.keys() {
        if directions.iter().all(|(dx, dy)| {
            let (nx, ny) = (x + dx, y + dy);
            map.get(&(nx, ny)).unwrap_or(&' ') == &'#' || map.get(&(nx, ny)).unwrap_or(&' ') == &'^'
        }) {
            sum += ((x * y) as i64).abs();
        }
    }
    sum
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

enum Dir {
    U,
    D,
    L,
    R,
}

impl Dir {
    fn left(&self) -> Dir {
        match self {
            Dir::U => Dir::L,
            Dir::D => Dir::R,
            Dir::L => Dir::D,
            Dir::R => Dir::U,
        }
    }

    fn right(&self) -> Dir {
        match self {
            Dir::U => Dir::R,
            Dir::D => Dir::L,
            Dir::L => Dir::U,
            Dir::R => Dir::D,
        }
    }

    fn coord(&self) -> (i64, i64) {
        match self {
            Dir::U => (0, -1),
            Dir::D => (0, 1),
            Dir::L => (-1, 0),
            Dir::R => (1, 0),
        }
    }
}

fn is_free(map: &HashMap<(i64, i64), char>, pos: &(i64, i64), dir: &Dir) -> bool {
    let dp = dir.coord();
    let np = (pos.0 + dp.0, pos.1 + dp.1);
    map.get(&np).unwrap_or(&' ') == &'#'
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Cmd {
    L,
    R,
    M(usize),
}

impl Cmd {
    fn to_str(&self) -> String {
        match self {
            &Cmd::L => "L".to_string(),
            &Cmd::R => "R".to_string(),
            &Cmd::M(n) => n.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Compression {
    a: Option<Vec<Cmd>>,
    b: Option<Vec<Cmd>>,
    c: Option<Vec<Cmd>>,
    stack: Vec<Cmd>,
    result: Vec<&'static str>,
}

impl Compression {
    fn commit(&self) -> Option<Compression> {
        if self.stack.len() < 5 {
            // is this ok?
            None
        } else if self.result.len() == 10 {
            return None;
        } else if self.a.is_none() {
            let mut other = self.clone();
            other.a = Some(other.stack.split_off(0));
            other.result.push("A");
            eprintln!("{:?}", &other);
            Some(other)
        } else if self.b.is_none() {
            let mut other = self.clone();
            other.b = Some(other.stack.split_off(0));
            other.result.push("B");
            eprintln!("{:?}", &other);
            Some(other)
        } else if self.c.is_none() {
            let mut other = self.clone();
            other.c = Some(other.stack.split_off(0));
            other.result.push("C");
            eprintln!("{:?}", &other);
            Some(other)
        } else {
            None
        }
    }

    fn eat(&self) -> Option<Compression> {
        if self.stack.is_empty() {
            return None;
        }

        if self.result.len() == 10 {
            return None;
        }

        for (i, s) in [&self.a, &self.b, &self.c].iter().enumerate() {
            if let Some(s) = s {
                if s == &self.stack {
                    let mut other = self.clone();
                    other.stack.clear();
                    if i == 0 {
                        other.result.push("A");
                    } else if i == 1 {
                        other.result.push("B");
                    } else if i == 2 {
                        other.result.push("C");
                    }
                    return Some(other);
                }
            }
        }

        None
    }
}

fn compress(stream: &[Cmd], mut c: Compression) -> Option<Compression> {
    if stream.is_empty() && c.stack.is_empty() {
        return Some(c);
    }
    if let Some(commit) = c.commit() {
        if let Some(soln) = compress(stream, commit) {
            return Some(soln);
        }
    }
    if let Some(eat) = c.eat() {
        if let Some(soln) = compress(stream, eat) {
            return Some(soln);
        }
    }
    if !stream.is_empty() {
        let cmd = stream[0];
        c.stack.push(cmd);
        if spell_cmd(&c.stack).len() <= 20 {
            if let Some(soln) = compress(&stream[1..], c.clone()) {
                return Some(soln);
            }
        }
        c.stack.pop();
    }

    None
}

fn spell_cmd(a: &Vec<Cmd>) -> String {
    a.iter()
        .map(|a| a.to_str())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn part_b(input: &str) -> i64 {
    let program: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let mut computer = IntComputer::new(&program);
    let mut x = 0;
    let mut y = 0;
    let mut map = HashMap::new();
    let mut pos = (0, 0);
    loop {
        computer.exec_once();
        if computer.finished() {
            break;
        }
        let out = computer.output.pop_front().unwrap();
        match out {
            35 => {
                map.insert((x, y), '#');
                x += 1;
            }
            46 => {
                map.insert((x, y), '.');
                x += 1;
            }
            10 => {
                x = 0;
                y += 1;
            }
            _ => {
                map.insert((x, y), '^');
                pos.0 = x;
                pos.1 = y;
                x += 1;
            }
        };
    }

    let mut stream = Vec::new();
    let mut steps = 0;
    let mut dir = Dir::U;

    loop {
        if is_free(&map, &pos, &dir) {
            steps += 1;
            let dp = dir.coord();
            pos.0 += dp.0;
            pos.1 += dp.1;
            continue;
        } else if steps > 0 {
            stream.push(Cmd::M(steps));
            steps = 0;
        }

        if is_free(&map, &pos, &dir.left()) {
            dir = dir.left();
            stream.push(Cmd::L);
            continue;
        }

        if is_free(&map, &pos, &dir.right()) {
            dir = dir.right();
            stream.push(Cmd::R);
            continue;
        }
        break;
    }

    eprintln!("{:?}", stream);
    let soln = compress(&stream, Default::default()).unwrap();
    eprintln!("{:?}", soln);
    let soln = format!(
        "{}\n{}\n{}\n{}\nn\n",
        soln.result.join(","),
        spell_cmd(&soln.a.unwrap()),
        spell_cmd(&soln.b.unwrap()),
        spell_cmd(&soln.c.unwrap())
    );

    // eprintln!("{}", soln);
    let mut computer = IntComputer::new(&program);
    *computer.mem.entry(0).or_insert(2) = 2;
    computer.input.extend(soln.chars().map(|c| c as i64));
    computer.exec();
    computer.output.pop_back().unwrap()

    // solve by hand
    // let mut computer = IntComputer::new(&program);
    // let main = "A,B,A,C,A,A,C,B,C,B\n";
    // let a = "L,12,L,8,R,12\n";
    // let b = "L,10,L,8,L,12,R,12\n";
    // let c = "R,12,L,8,L,10\n";
    // let continuous = "n\n";
    // for ch in main
    //     .chars()
    //     .chain(a.chars())
    //     .chain(b.chars())
    //     .chain(c.chars())
    //     .chain(continuous.chars())
    // {
    //     computer.input.push_back(ch as i64);
    // }

    // *computer.mem.entry(0).or_insert(2) = 2;
    // computer.exec();

    // computer.output.pop_back().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 10632);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 1356191);
    }
}
