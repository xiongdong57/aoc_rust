use std::collections::{HashMap, HashSet};

fn gen_dots(line: &Vec<&str>) -> HashSet<(i64, i64)> {
    let mut dots = HashSet::new();
    let mut cur_x = 0;
    let mut cur_y = 0;
    for step in line {
        let (direction, distance) = step.split_at(1);
        let mut dx = 0;
        let mut dy = 0;
        match direction {
            "U" => dy = 1,
            "D" => dy = -1,
            "L" => dx = -1,
            "R" => dx = 1,
            _ => panic!("Unknown direction"),
        }
        for _ in 0..distance.parse::<i64>().unwrap() {
            cur_x += dx;
            cur_y += dy;
            dots.insert((cur_x, cur_y));
        }
    }

    dots
}

pub fn part_a(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .collect::<Vec<_>>();
    let dots_a = gen_dots(&lines[0]);
    let dots_b = gen_dots(&lines[1]);

    dots_a
        .intersection(&dots_b)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap()
}

fn gen_dots_v2(line: &Vec<&str>) -> HashMap<(i64, i64), i64> {
    let mut dots = HashMap::new();
    let mut cur_x = 0;
    let mut cur_y = 0;
    let mut step = 0;
    for cur in line {
        let (direction, distance) = cur.split_at(1);
        let mut dx = 0;
        let mut dy = 0;
        match direction {
            "U" => dy = 1,
            "D" => dy = -1,
            "L" => dx = -1,
            "R" => dx = 1,
            _ => panic!("Unknown direction"),
        }
        for _ in 0..distance.parse::<i64>().unwrap() {
            cur_x += dx;
            cur_y += dy;
            step += 1;
            dots.insert((cur_x, cur_y), step);
        }
    }

    dots
}

pub fn part_b(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| line.split(",").collect::<Vec<&str>>())
        .collect::<Vec<_>>();
    let dots_a = gen_dots_v2(&lines[0]);
    let dots_b = gen_dots_v2(&lines[1]);

    let mut min_step = std::i64::MAX;
    for key in dots_a.keys() {
        if dots_b.contains_key(key) {
            min_step = min_step.min(dots_a[key].abs() + dots_b[key].abs());
        }
    }
    min_step
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 159);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 489);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 610);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 93654);
    }
}
