use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn eval_dirs(&self) -> [(i32, i32); 3] {
        match self {
            Self::North => [(-1, -1), (0, -1), (1, -1)],
            Self::South => [(-1, 1), (0, 1), (1, 1)],
            Self::West => [(-1, -1), (-1, 0), (-1, 1)],
            Self::East => [(1, -1), (1, 0), (1, 1)],
        }
    }
    fn eval_move(&self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::East => (1, 0),
        }
    }

    fn all_adjacent(&self) -> [(i32, i32); 8] {
        [
            (0, -1),
            (-1, 1),
            (0, 1),
            (1, 1),
            (-1, -1),
            (-1, 0),
            (1, -1),
            (1, 0),
        ]
    }

    fn next(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::West,
            Self::West => Self::East,
            Self::East => Self::North,
        }
    }
}

fn parse(input: &str) -> HashMap<(i32, i32), char> {
    let mut map = HashMap::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                map.insert((x as i32, y as i32), '#');
            }
        }
    }
    map
}

fn simulate(map: &mut HashMap<(i32, i32), char>, dir: Dir) -> HashMap<(i32, i32), char> {
    let mut proposes = HashMap::new();
    for loc in map.keys() {
        let all_free = !dir.all_adjacent().iter().any(|(dx, dy)| {
            let nx = loc.0 + dx;
            let ny = loc.1 + dy;
            map.contains_key(&(nx, ny))
        });
        if all_free {
            // not move
            proposes.insert(loc, *loc);
            continue;
        }
        let mut dir = dir;
        for _ in 0..4 {
            let free = !dir.eval_dirs().iter().any(|(dx, dy)| {
                let nx = loc.0 + dx;
                let ny = loc.1 + dy;
                map.contains_key(&(nx, ny))
            });
            if free {
                let nx = dir.eval_move().0 + loc.0;
                let ny = dir.eval_move().1 + loc.1;
                proposes.insert(loc, (nx, ny));
                break;
            }
            dir = dir.next();
        }
    }

    let mut new_map = HashMap::new();
    for k in map.keys() {
        if let Some(p) = proposes.get(k) {
            if proposes.values().filter(|vm| vm == &p).count() < 2 {
                // move
                new_map.insert(*p, '#');
            } else {
                // not move
                new_map.insert(*k, '#');
            }
        } else {
            // not move
            new_map.insert(*k, '#');
        }
    }
    new_map
}

pub fn part_a(input: &str) -> i64 {
    let mut map = parse(input);
    let dirs = vec![Dir::North, Dir::South, Dir::West, Dir::East];
    for i in 0..10 {
        let dir = dirs[i % dirs.len()];
        map = simulate(&mut map, dir);
    }

    let mut free_counts = 0;
    let (min_x, min_y) = map
        .keys()
        .fold((i32::MAX, i32::MAX), |(min_x, min_y), (x, y)| {
            (min_x.min(*x), min_y.min(*y))
        });
    let (max_x, max_y) = map
        .keys()
        .fold((i32::MIN, i32::MIN), |(max_x, max_y), (x, y)| {
            (max_x.max(*x), max_y.max(*y))
        });
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if Some(&'#') != map.get(&(x, y)) {
                free_counts += 1;
            }
        }
    }
    free_counts
}

pub fn part_b(input: &str) -> usize {
    let mut map = parse(input);
    let dirs = vec![Dir::North, Dir::South, Dir::West, Dir::East];
    for i in 0.. {
        let dir = dirs[i % dirs.len()];
        let new_map = simulate(&mut map, dir);
        if new_map == map {
            return i + 1;
        } else {
            map = new_map;
        }
    }
    panic!("no solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 110);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 3966);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 20);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 933);
        // slow, cost 374s to run, may be fix it later.
        // maybe use a HashSet instead of HashMap to store loc of elf.
    }
}
