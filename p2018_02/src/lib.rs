fn contains_n(line: &str, n: usize) -> bool {
    for c in line.chars() {
        if line.matches(c).count() == n {
            return true;
        }
    }
    false
}

pub fn part_a(input: &str) -> usize {
    let twos = input.lines().filter(|line| contains_n(line, 2)).count();
    let threes = input.lines().filter(|line| contains_n(line, 3)).count();
    twos * threes
}

fn diff_chars(a: &str, b: &str) -> usize {
    a.chars().zip(b.chars()).filter(|(a, b)| a != b).count()
}

pub fn part_b(input: &str) -> String {
    for (i, a) in input.lines().enumerate() {
        for b in input.lines().skip(i + 1) {
            if diff_chars(a, b) == 1 {
                let res = a
                    .chars()
                    .zip(b.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _)| a)
                    .into_iter()
                    .collect::<String>();
                return res;
            }
        }
    }
    panic!("No match found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 12);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 9139);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test2.txt");
        assert_eq!(part_b(input), "fgij");
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), "");
    }
}
