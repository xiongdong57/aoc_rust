pub fn part_a(input: &str) -> i64 {
    let up_floors = input.chars().filter(|&c| c == '(').count() as i64;
    let down_floors = input.chars().filter(|&c| c == ')').count() as i64;
    up_floors - down_floors
}

pub fn part_b(input: &str) -> usize {
    let mut cur_floors = 0;
    for (i, ch) in input.chars().enumerate() {
        match ch {
            '(' => cur_floors += 1,
            ')' => cur_floors -= 1,
            _ => panic!("Invalid character"),
        }

        if cur_floors == -1 {
            return i + 1;
        }
    }
    panic!("No basement found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = "))(";
        assert_eq!(part_a(input), -1);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 138);
    }

    #[test]
    fn part_b_works() {
        let input = "()())";
        assert_eq!(part_b(input), 5);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 1771);
    }
}
