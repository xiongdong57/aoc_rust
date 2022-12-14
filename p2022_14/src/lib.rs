use std::collections::HashMap;

fn parse(input: &str) -> HashMap<(i32, i32), char> {
    let mut map = HashMap::new();
    let mut y_m = 0;
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    for line in input.lines() {
        let mut sa = 0;
        let mut sb = 0;
        for p in line.split(" -> ") {
            let (a, b) = p.split_once(",").unwrap();
            let a = a.parse::<i32>().unwrap();
            let b = b.parse::<i32>().unwrap();
            if sa == 0 {
                sa = a;
                sb = b;
            } else if sa == a {
                // change b
                for i in b.min(sb)..=b.max(sb) {
                    map.insert((a, i), '#');
                }
                sb = b;
            } else if sb == b {
                // change a
                for i in a.min(sa)..=a.max(sa) {
                    map.insert((i, b), '#');
                }
                sa = a;
            } else {
                panic!("wrong rock loc")
            }
            y_m = y_m.max(b);
            x_max = a.max(x_max);
            x_min = a.min(x_min);
        }
    }
    for y in 0..=y_m {
        for x in x_min..=x_max {
            if !map.contains_key(&(x, y)) {
                map.insert((x, y), '.');
            }
        }
    }
    map
}

#[allow(dead_code)]
fn print_map(map: &HashMap<(i32, i32), char>) {
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
            print!("{}", map.get(&(x, y)).unwrap_or(&'.'));
        }
        println!();
    }
}

fn simulate(map: &mut HashMap<(i32, i32), char>, s: (i32, i32)) -> Result<(), &str> {
    let (x, y) = s;
    let down = map.get(&(x, y + 1));
    let down_is_stable = down == Some(&'#') || down == Some(&'o');

    let left = map.get(&(x - 1, y + 1));
    let left_is_stable = left == Some(&'#') || left == Some(&'o');

    let right = map.get(&(x + 1, y + 1));
    let right_is_stable = right == Some(&'#') || right == Some(&'o');

    if down.is_none() || left.is_none() || right.is_none() {
        return Err("overfolw");
    }
    if down_is_stable && left_is_stable && right_is_stable {
        *map.get_mut(&(x, y)).unwrap() = 'o';
        return Ok(());
    } else if !down_is_stable {
        simulate(map, (x, y + 1))
    } else if down_is_stable && !left_is_stable {
        simulate(map, (x - 1, y + 1))
    } else if down_is_stable && left_is_stable && !right_is_stable {
        simulate(map, (x + 1, y + 1))
    } else {
        return Err("");
    }
}

pub fn part_a(input: &str) -> i64 {
    let mut map = parse(input);
    for i in 0.. {
        if simulate(&mut map, (500, 0)).is_err() {
            return i;
        }
    }
    panic!()
}

fn simulate_v2(map: &mut HashMap<(i32, i32), char>, s: (i32, i32), max_y: i32) -> Result<(), &str> {
    let (x, y) = s;
    let down = map.get(&(x, y + 1));
    let down_is_stable = down == Some(&'#') || down == Some(&'o');

    let left = map.get(&(x - 1, y + 1));
    let left_is_stable = left == Some(&'#') || left == Some(&'o');

    let right = map.get(&(x + 1, y + 1));
    let right_is_stable = right == Some(&'#') || right == Some(&'o');
    if y >= max_y {
        return Err("overfolw");
    }
    if (y + 1) == max_y || (down_is_stable && left_is_stable && right_is_stable) {
        if map.get(&(x, y)) != Some(&'o') {
            *map.entry((x, y)).or_default() = 'o';
            return Ok(());
        } else {
            return Err("already filled");
        }
    } else if !down_is_stable {
        return simulate_v2(map, (x, y + 1), max_y);
    } else if down_is_stable && !left_is_stable {
        return simulate_v2(map, (x - 1, y + 1), max_y);
    } else if down_is_stable && left_is_stable && !right_is_stable {
        return simulate_v2(map, (x + 1, y + 1), max_y);
    } else {
        return Err("");
    }
}

pub fn part_b(input: &str) -> i64 {
    let mut map = parse(input);
    let max_y = *map.keys().map(|(_, x)| x).max().unwrap() + 2;

    for i in 0.. {
        if simulate_v2(&mut map, (500, 0), max_y).is_err() {
            return i;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 24);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 901);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 93);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 24589);
    }
}
