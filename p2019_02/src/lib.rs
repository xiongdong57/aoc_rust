fn run(nums: &Vec<i64>, noun: i64, verb: i64) -> Vec<i64> {
    let mut nums = nums.clone();
    nums[1] = noun;
    nums[2] = verb;
    let mut curr = 0;
    loop {
        let next = nums[curr as usize];
        match next {
            1 => {
                let p1 = nums[curr + 1];
                let p2 = nums[curr + 2];
                let p3 = nums[curr + 3];
                nums[p3 as usize] = nums[p1 as usize] + nums[p2 as usize];
                curr += 4;
            }
            2 => {
                let p1 = nums[curr + 1];
                let p2 = nums[curr + 2];
                let p3 = nums[curr + 3];
                nums[p3 as usize] = nums[p1 as usize] * nums[p2 as usize];
                curr += 4;
            }
            99 => return nums,
            _ => panic!("Invalid opcode"),
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let nums: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    run(&nums, 12, 2)[0]
}

pub fn part_b(input: &str) -> i64 {
    let nums: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    for noun in 0..=99 {
        for verb in 0..=99 {
            let result = run(&nums, noun, verb);
            if result[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 5110675);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 4847);
    }
}
