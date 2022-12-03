use std::ops::Div;

fn common(a: &str, b: &str) -> char {
    let bc = b.chars().collect::<Vec<char>>();
    for c in a.chars().into_iter() {
        if bc.contains(&c) {
            return c;
        }
    }
    panic!("no common char")
}

fn translate(c: char) -> usize {
    match c {
        'a'..='z' => (c as usize) - 96,
        'A'..='Z' => (c as usize) - 38,
        _ => panic!("no char"),
    }
}

pub fn part_a(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| line.split_at(line.len().div(2)))
        .map(|(a, b)| common(a, b))
        .map(|c| translate(c))
        .sum()
}

fn common_v2(a: &str, b: &str, c: &str) -> char {
    let bc = b.chars().collect::<Vec<char>>();
    let cc = c.chars().collect::<Vec<char>>();
    for c in a.chars().into_iter() {
        if bc.contains(&c) && cc.contains(&c) {
            return c;
        }
    }
    panic!("no common char")
}

pub fn part_b(input: &str) -> usize {
    let groups = input.trim().lines().collect::<Vec<&str>>();
    let mut score = 0;
    for i in (0..).step_by(3) {
        if i >= groups.len() {
            return score;
        }
        let (a, b, c) = (groups[i], groups[i + 1], groups[i + 2]);
        score += translate(common_v2(a, b, c))
    }
    panic!("no solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 157);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 7990);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 70);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 2602);
    }
}
