use std::collections::HashMap;

pub fn part_a(input: i64) -> i64 {
    let mut house = HashMap::new();
    let target = input;
    for i in 1..=target / 10 {
        for j in (i..=target / 10).step_by(i as usize) {
                *house.entry(j).or_insert(0) += 10 * i;
        }
    }

    let mut small_house = i64::MAX;
    for (k, v) in house {
        if v >= target {
            small_house = small_house.min(k);
        }
    }
    small_house
}

pub fn part_b(input: i64) -> i64 {
    let mut house = HashMap::new();
    let target = input;
    for i in 1..=target / 10 {
        for j in (i..=i*(50)).step_by(i as usize) {
                *house.entry(j).or_insert(0) += 11 * i;
        }
    }

    let mut small_house = i64::MAX;
    for (k, v) in house {
        if v >= target {
            small_house = small_house.min(k);
        }
    }
    small_house
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        assert_eq!(part_a(150), 8);
    }

    #[test]
    fn part_a_result() {
        assert_eq!(part_a(33100000), 776160);
    }


    #[test]
    fn part_b_result() {
        assert_eq!(part_b(33100000), 786240);
    }
}
