fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
        .count()
}

// contains at least one letter that appears twice in a row
fn contain_twice_chars(s: &str) -> bool {
    for (i, ch) in s.chars().enumerate() {
        if i + 1 < s.len() && ch == s.chars().nth(i + 1).unwrap() {
            return true;
        }
    }
    false
}

// not contain the strings ab, cd, pq, or xy
fn not_contain_string(s: &str) -> bool {
    let sub_strs = ["ab", "cd", "pq", "xy"];
    for sub_str in sub_strs.iter() {
        if s.contains(sub_str) {
            return false;
        }
    }
    true
}

pub fn part_a(input: &str) -> i64 {
    let mut count = 0;
    for line in input.lines() {
        if contain_twice_chars(line) && not_contain_string(line) && count_vowels(line) >= 3 {
            count += 1;
        }
    }
    count
}

// contains a pair of any two letters that appears at least twice
fn contain_twice_sub_str(s: &str) -> bool {
    for i in 0..s.len() - 1 {
        let sub_str = &s[i..i + 2];
        if s[i + 2..].contains(sub_str) {
            return true;
        }
    }
    false
}

// contains at least one letter which repeats with exactly one letter between them
fn contain_repeated_chars(s: &str) -> bool {
    for (i, ch) in s.chars().enumerate() {
        if i + 2 < s.len() && ch == s.chars().nth(i + 2).unwrap() {
            return true;
        }
    }
    false
}

pub fn part_b(input: &str) -> i64 {
    let mut count = 0;
    for line in input.lines() {
        if contain_twice_sub_str(line) && contain_repeated_chars(line) {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = "ugknbfddgicrmopn";
        assert_eq!(part_a(input), 1);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 255);
    }

    #[test]
    fn part_b_works() {
        let input = "xxyxx";
        assert_eq!(part_b(input), 1);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 55);
    }
}
