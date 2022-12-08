use std::collections::HashMap;

fn visible_dir(i: usize, j: usize, dx: i32, dy: i32, map: &HashMap<(usize, usize), i32>) -> bool {
    let n1 = map.get(&(i, j)).unwrap();
    let mut cx = i as i32;
    let mut cy = j as i32;
    loop {
        cx += dx;
        cy += dy;
        match map.get(&(cx as usize, cy as usize)) {
            Some(n2) => {
                if n1 <= n2 {
                    return false;
                } else {
                    continue;
                }
            }
            None => return true,
        }
    }
}

fn visible(i: usize, j: usize, map: &HashMap<(usize, usize), i32>, mx: usize, my: usize) -> bool {
    let dirs: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    if (i == 0) || (j == 0) || (i == mx) || (j == my) {
        return true;
    } else {
        dirs.iter()
            .any(|(dx, dy)| visible_dir(i, j, *dx, *dy, &map))
    }
}

pub fn part_a(input: &str) -> usize {
    let mut map = HashMap::new();
    let mut mx: usize = 0;
    let mut my: usize = 0;
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            map.insert((i, j), c.to_string().parse::<i32>().unwrap());
            mx = i;
            my = j;
        }
    }
    map.keys()
        .into_iter()
        .filter(|(i, j)| visible(*i, *j, &map, mx, my))
        .count()
}

fn count_visible(i: usize, j: usize, dx: i32, dy: i32, map: &HashMap<(usize, usize), i32>) -> i32 {
    let mut cnt = 0;
    let n1 = map.get(&(i, j)).unwrap();
    let mut cx = i as i32;
    let mut cy = j as i32;
    loop {
        cx += dx;
        cy += dy;
        match map.get(&(cx as usize, cy as usize)) {
            Some(n2) => {
                if n1 > n2 {
                    cnt += 1;
                } else {
                    cnt += 1;
                    break;
                }
            }
            None => break,
        }
    }
    cnt
}

fn sorce(i: usize, j: usize, map: &HashMap<(usize, usize), i32>, mx: usize, my: usize) -> usize {
    let dirs: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    if (i == 0) || (j == 0) || (i == mx) || (j == my) {
        return 0;
    } else {
        dirs.iter()
            .map(|(dx, dy)| count_visible(i, j, *dx, *dy, &map))
            .fold(1, |acc, x| acc * x as usize)
    }
}

pub fn part_b(input: &str) -> usize {
    let mut map = HashMap::new();
    let mut mx: usize = 0;
    let mut my: usize = 0;
    for (j, line) in input.lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            map.insert((i, j), c.to_string().parse::<i32>().unwrap());
            mx = i;
            my = j;
        }
    }

    map.keys()
        .into_iter()
        .map(|(i, j)| sorce(*i, *j, &map, mx, my))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 21);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 1669);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 8);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 331344);
    }
}
