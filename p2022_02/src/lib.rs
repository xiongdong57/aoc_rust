fn calc_score(a: &str, b: &str) -> usize {
    // a is ABC, b is XYZ
    // A for Rock, B for Paper, and C for Scissors
    // Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
    let n1 = vec!["A", "B", "C"].iter().position(|r| r == &a).unwrap() + 1;
    let n2 = vec!["X", "Y", "Z"].iter().position(|r| r == &b).unwrap() + 1;
    if n1 == n2 {
        // draw
        3 + n2
    } else if n1 + 1 == n2 || n1 == n2 + 2 {
        // won
        6 + n2
    } else {
        // lose
        0 + n2
    }
}

pub fn part_a(input: &str) -> usize {
    let mut score = 0;
    for line in input.split("\r\n") {
        let (a, b) = line.split_once(" ").unwrap();
        score += calc_score(a, b);
    }
    score
}

fn calc_score_v2(a: &str, b: &str) -> usize {
    let n1 = vec!["A", "B", "C"].iter().position(|r| r == &a).unwrap() + 1;
    if b == "X" {
        // lose
        0 + match n1 {
            1 => 3,
            _ => n1 - 1,
        }
    } else if b == "Y" {
        // draw
        3 + n1
    } else {
        // win
        6 + match n1 {
            3 => 1,
            _ => n1 + 1,
        }
    }
}

pub fn part_b(input: &str) -> usize {
    let mut score = 0;
    for line in input.split("\r\n") {
        let (a, b) = line.split_once(" ").unwrap();
        score += calc_score_v2(a, b);
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 15);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 12586);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 12);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 13193);
    }
}
