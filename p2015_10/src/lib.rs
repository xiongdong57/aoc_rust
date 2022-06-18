fn look_and_say(chars: Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut current_char = chars[0];
    let mut current_count = 1;
    for &c in &chars[1..] {
        if c == current_char {
            current_count += 1;
        } else {
            result.push(current_count as u8);
            result.push(current_char);
            current_char = c;
            current_count = 1;
        }
    }
    result.push(current_count as u8);
    result.push(current_char);
    result
}

pub fn part_a(input: &str) -> i64 {
    let mut chars = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    for _ in 0..40 {
        chars = look_and_say(chars);
    }

    chars.len() as i64
}

pub fn part_b(input: &str) -> i64 {
    let mut chars: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    for _ in 0..50 {
        chars = look_and_say(chars);
    }

    chars.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = "1113222113";
        assert_eq!(part_a(input), 252594);
    }

    #[test]
    fn part_b_result() {
        let input = "1113222113";
        assert_eq!(part_b(input), 3579328);
    }
}
