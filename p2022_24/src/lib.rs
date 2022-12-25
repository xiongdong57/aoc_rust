use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn eval_move(&self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}

fn parse(input: &str) -> Vec<((i32, i32), Dir)> {
    let mut map = Vec::new();
    for (y, line) in input.trim().lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let loc = (x as i32, y as i32);
            match ch {
                '^' => map.push((loc, Dir::Up)),
                'v' => map.push((loc, Dir::Down)),
                '<' => map.push((loc, Dir::Left)),
                '>' => map.push((loc, Dir::Right)),
                _ => (),
            };
        }
    }
    map
}

fn simulate(map: &mut Vec<((i32, i32), Dir)>, min_x: i32, max_x: i32, min_y: i32, max_y: i32) {
    for (loc, dir) in map.iter_mut() {
        let (dx, dy) = dir.eval_move();
        loc.0 = if (loc.0 + dx) < min_x {
            max_x
        } else if (loc.0 + dx) > max_x {
            min_x
        } else {
            loc.0 + dx
        };
        loc.1 = if (loc.1 + dy) < min_y {
            max_y
        } else if (loc.1 + dy) > max_y {
            min_y
        } else {
            loc.1 + dy
        };
    }
}

pub fn part_a(input: &str) -> i32 {
    let mut map = parse(input);
    let (min_x, min_y) = map
        .iter()
        .fold((i32::MAX, i32::MAX), |(mx, my), ((x, y), _)| {
            (mx.min(*x), my.min(*y))
        });
    let (max_x, max_y) = map
        .iter()
        .fold((i32::MIN, i32::MIN), |(mx, my), ((x, y), _)| {
            (mx.max(*x), my.max(*y))
        });
    let start = (min_x, min_y - 1);
    let goal = (max_x, max_y + 1);
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut cache = HashMap::new();
    let mut visited = HashSet::new();
    visited.insert((start, 0));
    while let Some(node) = queue.pop_front() {
        if node.0 == goal {
            return node.1;
        }

        if !cache.contains_key(&(node.1 + 1)) {
            simulate(&mut map, min_x, max_x, min_y, max_y);
            cache.insert(
                node.1 + 1,
                map.iter().map(|(l, _)| *l).collect::<HashSet<_>>(),
            );
        }
        let (x, y) = node.0;
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)] {
            let loc = (x + dx, y + dy);
            if !visited.contains(&(loc, node.1 + 1))
                && (0 < loc.0
                    && loc.0 <= max_x
                    && (0 < loc.1 || loc == start)
                    && (loc.1 <= max_y || loc == goal))
                && !cache.get(&(node.1 + 1)).unwrap().contains(&loc)
            {
                queue.push_back((loc, node.1 + 1));
                visited.insert((loc, node.1 + 1));
            }
        }
    }

    panic!("no solution")
}

pub fn part_b(input: &str) -> i32 {
    let mut map = parse(input);
    let (min_x, min_y) = map
        .iter()
        .fold((i32::MAX, i32::MAX), |(mx, my), ((x, y), _)| {
            (mx.min(*x), my.min(*y))
        });
    let (max_x, max_y) = map
        .iter()
        .fold((i32::MIN, i32::MIN), |(mx, my), ((x, y), _)| {
            (mx.max(*x), my.max(*y))
        });
    let start = (min_x, min_y - 1);
    let goal = (max_x, max_y + 1);

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut cache = HashMap::new();
    let mut visited = HashSet::new();
    visited.insert((start, 0));
    let mut t1 = 0;
    while let Some(node) = queue.pop_front() {
        if node.0 == goal {
            t1 = node.1;
            break;
        }

        if !cache.contains_key(&(node.1 + 1)) {
            simulate(&mut map, min_x, max_x, min_y, max_y);
            cache.insert(
                node.1 + 1,
                map.iter().map(|(l, _)| *l).collect::<HashSet<_>>(),
            );
        }
        let (x, y) = node.0;
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)] {
            let loc = (x + dx, y + dy);
            if !visited.contains(&(loc, node.1 + 1))
                && (0 < loc.0
                    && loc.0 <= max_x
                    && (0 < loc.1 || loc == start)
                    && (loc.1 <= max_y || loc == goal))
                && !cache.get(&(node.1 + 1)).unwrap().contains(&loc)
            {
                queue.push_back((loc, node.1 + 1));
                visited.insert((loc, node.1 + 1));
            }
        }
    }

    let mut t2 = 0;
    queue.clear();
    queue.push_back((goal, t1));
    while let Some(node) = queue.pop_front() {
        if node.0 == start {
            t2 = node.1;
            break;
        }

        if !cache.contains_key(&(node.1 + 1)) {
            simulate(&mut map, min_x, max_x, min_y, max_y);
            cache.insert(
                node.1 + 1,
                map.iter().map(|(l, _)| *l).collect::<HashSet<_>>(),
            );
        }
        let (x, y) = node.0;
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)] {
            let loc = (x + dx, y + dy);
            if !visited.contains(&(loc, node.1 + 1))
                && (0 < loc.0
                    && loc.0 <= max_x
                    && (0 < loc.1 || loc == start)
                    && (loc.1 <= max_y || loc == goal))
                && !cache.get(&(node.1 + 1)).unwrap().contains(&loc)
            {
                queue.push_back((loc, node.1 + 1));
                visited.insert((loc, node.1 + 1));
            }
        }
    }

    queue.clear();
    queue.push_back((start, t2));
    while let Some(node) = queue.pop_front() {
        if node.0 == goal {
            return node.1;
        }

        if !cache.contains_key(&(node.1 + 1)) {
            simulate(&mut map, min_x, max_x, min_y, max_y);
            cache.insert(
                node.1 + 1,
                map.iter().map(|(l, _)| *l).collect::<HashSet<_>>(),
            );
        }
        let (x, y) = node.0;
        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)] {
            let loc = (x + dx, y + dy);
            if !visited.contains(&(loc, node.1 + 1))
                && (0 < loc.0
                    && loc.0 <= max_x
                    && (0 < loc.1 || loc == start)
                    && (loc.1 <= max_y || loc == goal))
                && !cache.get(&(node.1 + 1)).unwrap().contains(&loc)
            {
                queue.push_back((loc, node.1 + 1));
                visited.insert((loc, node.1 + 1));
            }
        }
    }
    panic!("no sulution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 18);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 225);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 54);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 711);
    }
}
