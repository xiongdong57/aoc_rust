use md5;

pub fn part_a(input: &str) -> i64 {
    for i in 0.. {
        let hash = md5::compute(input.to_string() + &i.to_string());
        let digest = format!("{:x}", hash);
        if digest.starts_with("00000") {
            return i;
        }
    }
    panic!()
}

pub fn part_b(input: &str) -> i64 {
    for i in 0.. {
        let hash = md5::compute(input.to_string() + &i.to_string());
        let digest = format!("{:32x}", hash);
        if digest.starts_with("000000") {
            return i;
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = "abcdef";
        assert_eq!(part_a(input), 609043);
    }

    #[test]
    fn part_a_result() {
        let input = "yzbqklnj";
        assert_eq!(part_a(input), 282749);
    }

    #[test]
    fn part_b_result() {
        let input = "yzbqklnj";
        assert_eq!(part_b(input), 9962624);
    }
}
