use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let nums = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let balance: i64 = nums.iter().sum::<i64>() / 3;
    for i in 1..nums.len() {
        for group in nums.iter().combinations(i) {
            if group.iter().fold(0, |acc, x| acc + x.to_owned()) == balance {
                return group.iter().fold(1, |acc, x| acc * x.to_owned());
            }
        }
    }

    panic!("No solution found");
}

pub fn part_b(input: &str) -> i64 {
    let nums = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let balance: i64 = nums.iter().sum::<i64>() / 4;
    for i in 1..nums.len() {
        for group in nums.iter().combinations(i) {
            if group.iter().fold(0, |acc, x| acc + x.to_owned()) == balance {
                return group.iter().fold(1, |acc, x| acc * x.to_owned());
            }
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 99);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 11846773891);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 44);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 80393059);
    }
}
