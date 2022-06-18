pub fn part_a(input: &str) -> usize {
    let mut char_nums = 0;
    let mut mem_len = 0;
    for line in input.lines() {
        let mut skip_loc = 0;
        let mut memery_len = 0;
        for i in 0..line.len() - 1 {
            if skip_loc > 0 {
                skip_loc -= 1;
                continue;
            }

            memery_len += 1;
            let ch = line.chars().nth(i).unwrap();
            let ch2 = line.chars().nth(i + 1).unwrap();
            if ch == '\\' {
                if ch2 == '\\' || ch2 == '"' {
                    skip_loc += 1;
                } else if ch2 == 'x' {
                    skip_loc += 3;
                }
            }
        }
        char_nums += line.len();
        mem_len += memery_len - 1;
    }
    char_nums - mem_len
}

pub fn part_b(input: &str) -> usize {
    let mut diffs = 0;
    for line in input.lines() {
        let mut skip_loc = 0;
        let mut diff = 0;
        for i in 0..line.len() - 1 {
            if skip_loc > 0 {
                skip_loc -= 1;
                continue;
            }
            let ch = line.chars().nth(i).unwrap();
            let ch2 = line.chars().nth(i + 1).unwrap();
            if ch == '\\' {
                if ch2 == '\\' || ch2 == '"' {
                    diff += 2;
                    skip_loc += 1;
                } else if ch2 == 'x' {
                    diff += 1;
                    skip_loc += 3;
                }
            }
        }
        diffs += diff + 4;
    }
    diffs
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
        assert_eq!(part_a(input), 1333);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 19);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 2046);
    }
}
