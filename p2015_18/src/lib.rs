use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<(i64, i64), char> {
    let mut map = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            map.insert((i as i64, j as i64), ch);
        }
    }
    map
}

// game of life simulation
fn simulate(map: HashMap<(i64, i64), char>) -> HashMap<(i64, i64), char> {
    let neighbor: Vec<(i64, i64)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];
    let mut new_map: HashMap<(i64, i64), char> = HashMap::new();
    for ((x, y), v) in &map {
        let mut live_neighbor = 0;
        for (dx, dy) in &neighbor {
            let (nx, ny) = (x + *dx, y + *dy);
            if map.contains_key(&(nx, ny)) && map[&(nx, ny)] == '#' {
                live_neighbor += 1;
            }
        }

        if v == &'#' && (live_neighbor == 2 || live_neighbor == 3) {
            new_map.insert((*x, *y), '#');
        } else if v == &'.' && live_neighbor == 3 {
            new_map.insert((*x, *y), '#');
        } else {
            new_map.insert((*x, *y), '.');
        }
    }
    new_map
}

// allow unsed code
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

pub fn part_a(input: &str, simulate_times: i64) -> i64 {
    let mut map = parse_input(input);
    for _ in 0..simulate_times {
        map = simulate(map);
    }
    map.values().filter(|&v| v == &'#').count() as i64
}

pub fn keep_corners_light(map: &mut HashMap<(i64, i64), char>) {
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
    *map.entry((min_x, min_y)).or_insert('#') = '#';
    *map.entry((min_x, max_y)).or_insert('#') = '#';
    *map.entry((max_x, min_y)).or_insert('#') = '#';
    *map.entry((max_x, max_y)).or_insert('#') = '#';
}

pub fn part_b(input: &str, simulate_times: i64) -> i64 {
    let mut map = parse_input(input);
    keep_corners_light(&mut map);
    for _ in 0..simulate_times {
        // corners are always lights
        map = simulate(map);
        keep_corners_light(&mut map);
    }
    map.values().filter(|&v| v == &'#').count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input, 4), 4);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input, 100), 768);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input, 5), 17);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input, 100), 781);
    }
}
