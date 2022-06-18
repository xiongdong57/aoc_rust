pub fn part_a(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line.split("x").map(|n| n.parse::<i64>().unwrap()).collect();
            let (l, w, h) = (nums[0], nums[1], nums[2]);
            2 * l * w + 2 * w * h + 2 * h * l + (l * w).min(w * h).min(h * l)
        })
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let nums: Vec<i64> = line.split("x").map(|n| n.parse::<i64>().unwrap()).collect();
            let (l, w, h) = (nums[0], nums[1], nums[2]);
            l * w * h + 2 * (l + w).min(w + h).min(h + l)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = "1x1x10";
        assert_eq!(part_a(input), 43);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 1606483);
    }

    #[test]
    fn part_b_works() {
        let input = "2x3x4";
        assert_eq!(part_b(input), 34);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 3842356);
    }
}
