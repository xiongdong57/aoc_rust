use std::cmp::{Ordering, Reverse};
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
struct Angle {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        self.angle() == other.angle()
    }
}

impl Eq for Angle {}

impl Ord for Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.angle().partial_cmp(&other.angle()).unwrap()
    }
}

impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.angle().partial_cmp(&other.angle())
    }
}

impl Angle {
    fn new(p1: (i32, i32), p2: (i32, i32)) -> Self {
        Self {
            x0: p1.0,
            y0: p1.1,
            x1: p2.0,
            y1: p2.1,
        }
    }

    fn angle(&self) -> f64 {
        let ang = (self.y1 as f64 - self.y0 as f64).atan2(self.x1 as f64 - self.x0 as f64);
        match ang {
            i if i > PI / 2f64 && i <= PI => i - 2f64 * PI,
            _ => ang,
        }
    }
}

fn parse_input(input: &str) -> Vec<(i32, i32)> {
    let mut dots = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                dots.push((j as i32, -(i as i32)));
            }
        }
    }
    dots
}

fn count_straight_line(dots: &Vec<(i32, i32)>, point: (i32, i32)) -> usize {
    let mut seen_angels = Vec::new();
    let mut result = 0;
    for dot in dots {
        if dot != &point {
            let curr_angle = Angle::new(point, *dot);
            if !seen_angels.contains(&curr_angle) {
                result += 1;
                seen_angels.push(curr_angle);
            }
        }
    }

    result
}
pub fn part_a(input: &str) -> usize {
    let dots = parse_input(input);
    let mut max_seen = usize::MIN;
    for dot in &dots {
        let count = count_straight_line(&dots, *dot);
        if count > max_seen {
            max_seen = count;
        }
    }

    max_seen
}

pub fn part_b(input: &str) -> i32 {
    let dots = parse_input(input);
    let mut max_seen = usize::MIN;
    let mut loc = (0, 0);
    for dot in &dots {
        let count = count_straight_line(&dots, *dot);
        if count > max_seen {
            max_seen = count;
            loc = *dot;
        }
    }
    let mut count = 0;
    let mut seen = Vec::new();
    let mut angs = Vec::new();
    for dot in &dots {
        if dot != &loc {
            let ang = Angle::new(loc, *dot);
            let distance = (dot.0 - loc.0).abs() + (dot.1 - loc.1).abs();
            angs.push((ang, Reverse(distance)));
        }
    }
    angs.sort();
    loop {
        for i in (0..angs.len()).rev() {
            let ang = angs[i].0;
            if !seen.contains(&ang) {
                count += 1;
                if count == 200 {
                    return ang.x1.abs() * 100 + ang.y1.abs();
                }
                seen.push(ang);
            }
        }
        seen.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 8);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 278);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test2.txt");
        assert_eq!(part_b(input), 802);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 1417);
    }
}
