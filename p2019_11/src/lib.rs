#[derive(Debug, Clone, Copy)]
struct Moon {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl PartialEq for Moon {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.z == other.z
            && self.vx == other.vx
            && self.vy == other.vy
            && self.vz == other.vz
    }
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            x,
            y,
            z,
            vx: 0,
            vy: 0,
            vz: 0,
        }
    }

    fn move_once(&mut self, moons: &Vec<Moon>) {
        for axis in ["x", "y", "z"] {
            self.move_axis(moons, axis);
        }
    }

    fn move_axis(&mut self, moons: &Vec<Moon>, axis: &str) {
        self.update_vel(moons, axis);
        self.update_loc(axis);
    }

    fn update_vel(&mut self, moons: &Vec<Moon>, axis: &str) {
        match axis {
            "x" => self.vx += moons.iter().map(|m| calc_step(self.x, m.x)).sum::<i64>(),
            "y" => self.vy += moons.iter().map(|m| calc_step(self.y, m.y)).sum::<i64>(),
            "z" => self.vz += moons.iter().map(|m| calc_step(self.z, m.z)).sum::<i64>(),
            _ => panic!("no such axis: {}", axis),
        }
    }
    fn update_loc(&mut self, axis: &str) {
        match axis {
            "x" => self.x += self.vx,
            "y" => self.y += self.vy,
            "z" => self.z += self.vz,
            _ => panic!("no such axis: {}", axis),
        }
    }

    fn energy(&self) -> i64 {
        let pot = self.x.abs() + self.y.abs() + self.z.abs();
        let bin = self.vx.abs() + self.vy.abs() + self.vz.abs();
        pot * bin
    }
}

fn calc_step(x0: i64, x1: i64) -> i64 {
    match x1 - x0 {
        0 => 0,
        i if i > 0 => 1,
        _ => -1,
    }
}

fn parse_input(input: &str) -> Vec<Moon> {
    let mut res = Vec::new();
    for line in input.lines() {
        let parts: Vec<&str> = line[1..line.len() - 1].split(",").collect();
        let x = parts[0].split("=").collect::<Vec<&str>>()[1]
            .parse::<i64>()
            .unwrap();
        let y = parts[1].split("=").collect::<Vec<&str>>()[1]
            .parse::<i64>()
            .unwrap();
        let z = parts[2].split("=").collect::<Vec<&str>>()[1]
            .parse::<i64>()
            .unwrap();
        res.push(Moon::new(x, y, z));
    }
    res
}

fn simulate(moons: &mut Vec<Moon>) {
    let origin = moons.clone();
    for moon in moons.iter_mut() {
        moon.move_once(&origin);
    }
}

pub fn part_a(input: &str, steps: i64) -> i64 {
    let mut moons = parse_input(input);
    for _ in 0..steps {
        simulate(&mut moons);
    }
    moons.iter().map(|m| m.energy()).sum()
}

fn until_repeat(moons: &mut Vec<Moon>, axis: &str) -> i64 {
    let origin = moons.clone();
    let mut count = 0;
    loop {
        if count > 0
            && origin[0] == moons[0]
            && origin[1] == moons[1]
            && origin[2] == moons[2]
            && origin[3] == moons[3]
        {
            return count;
        }
        let curr_moons = moons.clone();
        for moon in moons.iter_mut() {
            moon.move_axis(&curr_moons, axis);
        }
        count += 1;
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    let mut x;
    let mut y;
    if a > b {
        x = a;
        y = b;
    } else {
        x = b;
        y = a;
    }
    let mut rem = x % y;
    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }
    a * b / y
}

pub fn part_b(input: &str) -> i64 {
    let mut moons = parse_input(input);
    let rx = until_repeat(&mut moons, "x");
    let ry = until_repeat(&mut moons, "y");
    let rz = until_repeat(&mut moons, "z");

    lcm(lcm(rx, ry), rz)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input, 100), 1940);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input, 1000), 8044);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test2.txt");
        assert_eq!(part_b(input), 4686774924);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 362375881472136);
    }
}
