use std::collections::HashSet;

pub fn part_a(input: &str) -> i64 {
    input
        .lines()
        .map(|l| l.parse::<i64>().expect("parse line error"))
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let mut curr = 0;
    let mut history = HashSet::new();
    history.insert(curr);
    let mut iter = input.lines();
    loop {
        if let Some(line) = iter.next() {
            let num = line.parse::<i64>().expect("parse line error");
            curr += num;
            if history.contains(&curr) {
                return curr;
            }
            history.insert(curr);
        } else {
            iter = input.lines();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), -6);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 556);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test2.txt");
        assert_eq!(part_b(input), 14);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 0);
    }
}
