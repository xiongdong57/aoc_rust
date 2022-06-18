fn run(nums: &Vec<i64>, input: i64) -> i64 {
    let mut nums = nums.clone();
    let mut curr = 0;
    let mut output = 0;
    loop {
        let next = format!("{:0>5}", nums[curr as usize]);
        // eg: 01002
        let opcode = next[3..5].parse::<i64>().unwrap();
        let mode1 = next[2..3].parse::<i64>().unwrap();
        let mode2 = next[1..2].parse::<i64>().unwrap();
        match opcode {
            1 => {
                let v1 = match mode1 {
                    0 => nums[nums[curr + 1] as usize],
                    1 => nums[curr + 1],
                    _ => panic!("Invalid mode"),
                };
                let v2 = match mode2 {
                    0 => nums[nums[curr + 2] as usize],
                    1 => nums[curr + 2],
                    _ => panic!("Invalid mode"),
                };
                let p3 = nums[curr + 3];
                nums[p3 as usize] = v1 + v2;
                curr += 4;
            }
            2 => {
                let v1 = match mode1 {
                    0 => nums[nums[curr + 1] as usize],
                    1 => nums[curr + 1],
                    _ => panic!("Invalid mode"),
                };
                let v2 = match mode2 {
                    0 => nums[nums[curr + 2] as usize],
                    1 => nums[curr + 2],
                    _ => panic!("Invalid mode"),
                };
                let p3 = nums[curr + 3];
                nums[p3 as usize] = v1 * v2;
                curr += 4;
            }
            3 => {
                match mode1 {
                    0 => {
                        let p = nums[curr + 1];
                        nums[p as usize] = input;
                    }
                    _ => panic!("Invalid mode"),
                };
                curr += 2;
            }
            4 => {
                match mode1 {
                    0 => {
                        let p = nums[curr + 1];
                        output = nums[p as usize];
                    }
                    1 => {
                        output = nums[curr + 1];
                    }
                    _ => panic!("Invalid mode"),
                }
                curr += 2;
            }
            5 => {
                let v1 = match mode1 {
                    0 => nums[nums[curr + 1] as usize],
                    1 => nums[curr + 1],
                    _ => panic!("Invalid mode"),
                };
                let v2: i64 = match mode2 {
                    0 => nums[nums[curr + 2] as usize],
                    1 => nums[curr + 2],
                    _ => panic!("Invalid mode"),
                };
                match v1 != 0 {
                    true => curr = v2 as usize,
                    false => curr += 3,
                }
            }
            6 => {
                let v1 = match mode1 {
                    0 => nums[nums[curr + 1] as usize],
                    1 => nums[curr + 1],
                    _ => panic!("Invalid mode"),
                };
                let v2: i64 = match mode2 {
                    0 => nums[nums[curr + 2] as usize],
                    1 => nums[curr + 2],
                    _ => panic!("Invalid mode"),
                };
                match v1 {
                    0 => curr = v2 as usize,
                    _ => curr += 3,
                }
            }
            7 => {
                let v1 = match mode1 {
                    0 => nums[nums[curr + 1] as usize],
                    1 => nums[curr + 1],
                    _ => panic!("Invalid mode"),
                };
                let v2: i64 = match mode2 {
                    0 => nums[nums[curr + 2] as usize],
                    1 => nums[curr + 2],
                    _ => panic!("Invalid mode"),
                };
                let p3 = nums[curr + 3];
                match v1 < v2 {
                    true => nums[p3 as usize] = 1,
                    false => nums[p3 as usize] = 0,
                };
                curr += 4;
            }
            8 => {
                let v1 = match mode1 {
                    0 => nums[nums[curr + 1] as usize],
                    1 => nums[curr + 1],
                    _ => panic!("Invalid mode"),
                };
                let v2: i64 = match mode2 {
                    0 => nums[nums[curr + 2] as usize],
                    1 => nums[curr + 2],
                    _ => panic!("Invalid mode"),
                };
                let p3 = nums[curr + 3];
                match v1 == v2 {
                    true => nums[p3 as usize] = 1,
                    false => nums[p3 as usize] = 0,
                };
                curr += 4;
            }
            99 => return output,
            _ => panic!("Invalid opcode: {}", next),
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let nums: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    run(&nums, 1)
}

pub fn part_b(program: &str, input: i64) -> i64 {
    let nums: Vec<i64> = program
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    run(&nums, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 16348437);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input, 3), 1);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input, 5), 6959377);
    }
}
