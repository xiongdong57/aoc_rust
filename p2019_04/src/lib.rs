fn adjace_same(pwd: &Vec<char>) -> bool {
    for i in 0..pwd.len() - 1 {
        if pwd[i] == pwd[i + 1] {
            return true;
        }
    }
    false
}

fn is_decreasing(pwd: &Vec<char>) -> bool {
    for i in 0..pwd.len() - 1 {
        if pwd[i] > pwd[i + 1] {
            return true;
        }
    }
    false
}

fn valid(pwd: &str) -> bool {
    let pwd = pwd.chars().collect::<Vec<char>>();
    adjace_same(&pwd) && !is_decreasing(&pwd)
}

pub fn part_a() -> i64 {
    (357253..=892942)
        .map(|i| i.to_string())
        .filter(|pwd| valid(pwd))
        .count() as i64
}

fn adjace_same_v2(pwd: &Vec<char>) -> bool {
    for i in 0..pwd.len() {
        let mut count = 0;
        for j in 0..pwd.len() {
            if pwd[j] == pwd[i] {
                count += 1;
            }
        }
        if count == 2 {
            return true;
        }
    }
    false
}

fn valid_v2(pwd: &str) -> bool {
    let pwd = pwd.chars().collect::<Vec<char>>();
    adjace_same_v2(&pwd) && !is_decreasing(&pwd)
}

pub fn part_b() -> i64 {
    (357253..=892942)
        .map(|i| i.to_string())
        .filter(|pwd| valid_v2(pwd))
        .count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        assert!(valid("111111"));
        assert!(!valid("223450"));
        assert!(!valid("123789"));
    }

    #[test]
    fn part_a_result() {
        assert_eq!(part_a(), 530);
    }

    #[test]
    fn part_b_works() {
        assert!(valid_v2("112233"));
        assert!(!valid_v2("123444"));
        assert!(valid_v2("111122"));
    }

    #[test]
    fn part_b_result() {
        assert_eq!(part_b(), 324);
    }
}
