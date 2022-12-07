fn unique(chars: Vec<char>) -> bool {
    chars
        .iter()
        .all(|c| chars.iter().filter(|r| r == &c).count() == 1)
}

pub fn part_a(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    for i in 3..chars.len() {
        if unique(chars[i - 3..=i].to_vec()) {
            return i + 1;
        }
    }
    panic!("no such position")
}

pub fn part_b(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    for i in 13..chars.len() {
        if unique(chars[i - 13..=i].to_vec()) {
            return i + 1;
        }
    }
    panic!("no such position")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn func() {
        assert!(unique(vec!['j', 'p', 'q', 'm']));
    }

    #[test]
    fn part_a_works() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(part_a(input), 11);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 1175);
    }

    #[test]
    fn part_b_works() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(part_b(input), 19);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 3217);
    }
}
