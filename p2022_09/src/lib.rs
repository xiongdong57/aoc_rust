use std::collections::{HashMap, HashSet};

fn calculate_tail(t: (i32, i32), h: (i32, i32)) -> (i32, i32) {
    if ((t.0 - h.0).abs() <= 1) && ((t.1 - h.1).abs() <= 1) {
        // tail not move
        return t;
    }

    if t.0 == h.0 {
        // same row
        if h.1 > t.1 {
            return (t.0, t.1 + 1);
        } else {
            return (t.0, t.1 - 1);
        }
    }

    if t.1 == h.1 {
        // same column
        if h.0 > t.0 {
            return (t.0 + 1, t.1);
        } else {
            return (t.0 - 1, t.1);
        }
    }

    // move diagonally
    if h.0 > t.0 && h.1 > t.1 {
        return (t.0 + 1, t.1 + 1);
    } else if h.0 > t.0 && h.1 < t.1 {
        return (t.0 + 1, t.1 - 1);
    } else if h.0 < t.0 && h.1 > t.1 {
        return (t.0 - 1, t.1 + 1);
    } else {
        return (t.0 - 1, t.1 - 1);
    }
}

pub fn part_a(input: &str) -> usize {
    let mut map = HashMap::new();
    map.insert("R", (1, 0));
    map.insert("L", (-1, 0));
    map.insert("U", (0, 1));
    map.insert("D", (0, -1));
    let mut tl = Vec::new();
    let mut h = (0, 0);
    let mut t = (0, 0);
    for line in input.lines() {
        let (d, cnt) = line.split_once(" ").unwrap();
        let (dx, dy) = map.get(d).unwrap();
        for _ in 0..cnt.parse::<i32>().unwrap() {
            h = (h.0 + dx, h.1 + dy);
            t = calculate_tail(t, h);
            tl.push(t.clone());
        }
    }

    let unique_tls: HashSet<(i32, i32)> = HashSet::from_iter(tl);
    unique_tls.len()
}

pub fn part_b(input: &str) -> usize {
    let mut map = HashMap::new();
    map.insert("R", (1, 0));
    map.insert("L", (-1, 0));
    map.insert("U", (0, 1));
    map.insert("D", (0, -1));
    let mut tl = Vec::new();
    let mut h = (0, 0);
    let mut h1 = (0, 0);
    let mut h2 = (0, 0);
    let mut h3 = (0, 0);
    let mut h4 = (0, 0);
    let mut h5 = (0, 0);
    let mut h6 = (0, 0);
    let mut h7 = (0, 0);
    let mut h8 = (0, 0);
    let mut t = (0, 0);
    for line in input.lines() {
        let (d, cnt) = line.split_once(" ").unwrap();
        let (dx, dy) = map.get(d).unwrap();
        for _ in 0..cnt.parse::<i32>().unwrap() {
            h = (h.0 + dx, h.1 + dy);
            h1 = calculate_tail(h1, h);
            h2 = calculate_tail(h2, h1);
            h3 = calculate_tail(h3, h2);
            h4 = calculate_tail(h4, h3);
            h5 = calculate_tail(h5, h4);
            h6 = calculate_tail(h6, h5);
            h7 = calculate_tail(h7, h6);
            h8 = calculate_tail(h8, h7);
            t = calculate_tail(t, h8);
            tl.push(t.clone());
        }
    }

    let unique_tls: HashSet<(i32, i32)> = HashSet::from_iter(tl);
    unique_tls.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 13);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 6030);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test2.txt");
        assert_eq!(part_b(input), 36);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 2545);
    }
}
