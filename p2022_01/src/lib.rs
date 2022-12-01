use std::cmp::Reverse;

pub fn part_a(input: &str) -> i64 {
    let energy = input
        .split("\r\n\r\n")
        .map(|lines| {
            lines
                .split("\r\n")
                .map(|x| x.parse::<i32>().unwrap())
                .fold(0, |acc, x| acc + x)
        })
        .collect::<Vec<i32>>();
    *energy.iter().max().unwrap() as i64
}

pub fn part_b(input: &str) -> i64 {
    let mut energy = input
        .split("\r\n\r\n")
        .map(|lines| {
            lines
                .split("\r\n")
                .map(|x| x.parse::<i32>().unwrap())
                .fold(0, |acc, x| acc + x)
        })
        .collect::<Vec<i32>>();
    energy.sort_by_key(|w| Reverse(*w));
    energy.iter().take(3).fold(0, |acc, x| acc + (*x as i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 24000);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 70764);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 45000);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 203905);
    }
}
