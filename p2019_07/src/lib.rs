use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct IntComputer {
    program: Vec<i64>,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    curr: usize,
    exit: bool,
}

fn resolve(nums: &Vec<i64>, mode: i64, pos: usize) -> i64 {
    match mode {
        0 => nums[nums[pos] as usize],
        1 => nums[pos],
        _ => panic!("Invalid mode"),
    }
}
impl IntComputer {
    fn new(progrm: &Vec<i64>) -> Self {
        IntComputer {
            program: progrm.clone(),
            input: VecDeque::new(),
            output: VecDeque::new(),
            curr: 0,
            exit: false,
        }
    }

    fn exec_one(&mut self) {
        if self.exit {
            return;
        }
        let nums = &mut self.program;
        let mut curr = self.curr;
        loop {
            let next = format!("{:0>5}", nums[curr as usize]);
            // eg: 01002
            let opcode = next[3..5].parse::<i64>().unwrap();
            let mode1 = next[2..3].parse::<i64>().unwrap();
            let mode2 = next[1..2].parse::<i64>().unwrap();
            match opcode {
                1 => {
                    let v1 = resolve(nums, mode1, curr + 1);
                    let v2 = resolve(nums, mode2, curr + 2);
                    let p3 = nums[curr + 3];
                    nums[p3 as usize] = v1 + v2;
                    curr += 4;
                }
                2 => {
                    let v1 = resolve(nums, mode1, curr + 1);
                    let v2 = resolve(nums, mode2, curr + 2);
                    let p3 = nums[curr + 3];
                    nums[p3 as usize] = v1 * v2;
                    curr += 4;
                }
                3 => {
                    let p1 = nums[curr + 1] as usize;
                    nums[p1] = self.input.pop_front().unwrap();
                    curr += 2;
                }
                4 => {
                    self.output.push_back(resolve(nums, mode1, curr + 1));
                    curr += 2;
                    break;
                }
                5 => {
                    let v1 = resolve(nums, mode1, curr + 1);
                    let v2 = resolve(nums, mode2, curr + 2);
                    match v1 {
                        i if i != 0 => curr = v2 as usize,
                        _ => curr += 3,
                    }
                }
                6 => {
                    let v1 = resolve(nums, mode1, curr + 1);
                    let v2 = resolve(nums, mode2, curr + 2);
                    match v1 {
                        0 => curr = v2 as usize,
                        _ => curr += 3,
                    }
                }
                7 => {
                    let v1 = resolve(nums, mode1, curr + 1);
                    let v2 = resolve(nums, mode2, curr + 2);
                    let p3 = nums[curr + 3];
                    match v1 {
                        i if i < v2 => nums[p3 as usize] = 1,
                        _ => nums[p3 as usize] = 0,
                    };
                    curr += 4;
                }
                8 => {
                    let v1 = resolve(nums, mode1, curr + 1);
                    let v2 = resolve(nums, mode2, curr + 2);
                    let p3 = nums[curr + 3];
                    match v1 {
                        i if i == v2 => nums[p3 as usize] = 1,
                        _ => nums[p3 as usize] = 0,
                    };
                    curr += 4;
                }
                99 => {
                    self.exit = true;
                    break;
                }
                _ => panic!("Invalid opcode: {}", next),
            }
        }
        self.curr = curr;
    }

    fn finished(&self) -> bool {
        self.exit
    }
}

pub fn part_a(input: &str) -> i64 {
    let program: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    let signal = 0;
    let mut max_signal = i64::MIN;

    for group in (0..=4).permutations(5) {
        let mut amp1 = IntComputer::new(&program);
        let mut amp2 = IntComputer::new(&program);
        let mut amp3 = IntComputer::new(&program);
        let mut amp4 = IntComputer::new(&program);
        let mut amp5 = IntComputer::new(&program);

        let (i1, i2, i3, i4, i5) = (group[0], group[1], group[2], group[3], group[4]);

        amp1.input.push_back(i1);
        amp1.input.push_back(signal);
        amp2.input.push_back(i2);
        amp3.input.push_back(i3);
        amp4.input.push_back(i4);
        amp5.input.push_back(i5);

        amp1.exec_one();
        let o1 = amp1.output.pop_front().unwrap();

        amp2.input.push_back(o1);
        amp2.exec_one();
        let o2 = amp2.output.pop_front().unwrap();

        amp3.input.push_back(o2);
        amp3.exec_one();
        let o3 = amp3.output.pop_front().unwrap();

        amp4.input.push_back(o3);
        amp4.exec_one();
        let o4 = amp4.output.pop_front().unwrap();

        amp5.input.push_back(o4);
        amp5.exec_one();
        let o5 = amp5.output.pop_front().unwrap();

        max_signal = max_signal.max(o5);
    }
    max_signal
}

pub fn part_b(input: &str) -> i64 {
    let program: Vec<i64> = input
        .split(",")
        .map(|l| l.parse::<i64>().unwrap())
        .collect();

    let signal = 0;
    let mut max_signal = i64::MIN;
    for group in (5..=9).permutations(5) {
        // {
        let mut amp1 = IntComputer::new(&program);
        let mut amp2 = IntComputer::new(&program);
        let mut amp3 = IntComputer::new(&program);
        let mut amp4 = IntComputer::new(&program);
        let mut amp5 = IntComputer::new(&program);
        // let group = vec![9, 7, 8, 5, 6];
        let (i1, i2, i3, i4, i5) = (group[0], group[1], group[2], group[3], group[4]);
        amp1.input.push_back(i1);
        amp1.input.push_back(signal);
        amp2.input.push_back(i2);
        amp3.input.push_back(i3);
        amp4.input.push_back(i4);
        amp5.input.push_back(i5);

        loop {
            amp1.exec_one();
            if amp1.finished() {
                break;
            }

            let o1 = amp1.output.pop_front().unwrap();

            amp2.input.push_back(o1);
            amp2.exec_one();

            let o2 = amp2.output.pop_front().unwrap();

            amp3.input.push_back(o2);
            amp3.exec_one();

            let o3 = amp3.output.pop_front().unwrap();

            amp4.input.push_back(o3);
            amp4.exec_one();

            let o4 = amp4.output.pop_front().unwrap();

            amp5.input.push_back(o4);
            amp5.exec_one();
            let o5 = amp5.output.pop_front().unwrap();

            max_signal = max_signal.max(o5);

            amp1.input.push_back(o5);
        }
    }
    max_signal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        assert_eq!(
            part_a("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"),
            43210
        );
        assert_eq!(
            part_a("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"),
            54321
        );
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 45730);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 139629729);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 5406484);
    }
}
