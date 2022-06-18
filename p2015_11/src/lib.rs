use std::collections::HashSet;

pub fn passwd_increase(input: &str) -> String {
    let length = input.len();
    let mut chars: Vec<u8> = input.chars().map(|c| c as u8).collect();
    chars[length - 1] += 1;
    for i in (0..length).rev() {
        let ch = chars[i];
        if ch > 'z' as u8 {
            // overflow
            chars[i] = 'a' as u8;
            // previous char + 1
            if i > 0 {
                chars[i - 1] += 1;
            }
        }
    }
    chars.into_iter().map(|c| c as char).collect::<String>()
}

pub fn is_increasing_straight(input: &str) -> bool {
    let chars: Vec<u8> = input.chars().map(|c| c as u8).collect();
    for i in 0..chars.len() - 1 - 2 {
        if chars[i] + 1 == chars[i + 1] && chars[i] + 2 == chars[i + 2] {
            return true;
        }
    }
    false
}

pub fn is_not_specail(input: &str) -> bool {
    !['i', 'o', 'l'].iter().any(|&c| input.contains(c))
}

// contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
pub fn contains_two_pairs(input: &str) -> bool {
    let chars: Vec<u8> = input.chars().map(|c| c as u8).collect();
    let mut pairs = HashSet::new();
    for i in 0..input.len() - 1 {
        if chars[i] == chars[i + 1] {
            pairs.insert(chars[i]);
        }
    }
    pairs.len() >= 2
}

pub fn part_a(input: &str) -> String {
    let mut passwd = input.to_owned();
    loop {
        passwd = passwd_increase(&passwd);
        if is_not_specail(&passwd) && is_increasing_straight(&passwd) && contains_two_pairs(&passwd)
        {
            return passwd;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        assert_eq!(passwd_increase("ya"), "yb");
        assert_eq!(passwd_increase("xz"), "ya");
        assert!(is_increasing_straight("hijklmmn"));
        assert!(contains_two_pairs("abbceffg"));
        assert!(!contains_two_pairs("abbcegjk"));
    }

    #[test]
    fn part_a_result() {
        assert_eq!(part_a("hepxcrrq"), "hepxxyzz");
    }

    #[test]
    fn part_b_result() {
        assert_eq!(part_a("hepxxyzz"), "heqaabcc");
    }
}
