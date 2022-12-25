use ::std::collections::HashMap;
use std::ops::Rem;

fn snafu_to_num(line: &str) -> i64 {
    let mut map = HashMap::new();
    map.insert('2', 2);
    map.insert('1', 1);
    map.insert('0', 0);
    map.insert('-', -1);
    map.insert('=', -2);
    let mut num = 0;
    for (i, c) in line.trim().chars().rev().enumerate() {
        num += map[&c] * (5_i64.pow(i as u32));
    }
    num
}

fn num_to_snafu(num: i64) -> String {
    let mut map = HashMap::new();
    map.insert(2, "2");
    map.insert(1, "1");
    map.insert(0, "0");
    map.insert(-1, "-");
    map.insert(-2, "=");
    let mut rem = num.rem(5);
    let mut divided = num / 5;
    let mut chars = Vec::new();
    loop {
        let ch = if rem < 3 {
            map[&rem]
        } else {
            rem = rem - 5;
            divided += 1;
            map[&rem]
        };
        chars.push(ch);
        if divided == 0 {
            break;
        }
        rem = divided.rem(5);
        divided = divided / 5;
    }
    chars.reverse();
    chars.join("").to_string()
}

pub fn part_a(input: &str) -> String {
    let num: i64 = input.trim().lines().map(snafu_to_num).sum();
    num_to_snafu(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tranfer_works() {
        assert_eq!(snafu_to_num("1="), 3);
        assert_eq!(snafu_to_num("1=-0-2"), 1747);
    }

    #[test]
    fn reverse_works() {
        assert_eq!(num_to_snafu(107), "1-12");
    }

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), "2=-1=0");
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), "2-0=11=-0-2-1==1=-22");
    }
}
