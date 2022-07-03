fn get_pattern_val(position: usize, offset: usize) -> i32 {
    let base = [0, 1, 0, -1];
    if offset < position {
        return base[0];
    }
    let offset = offset - position;
    let idx = (offset / (position + 1) + 1) % 4;
    base[idx]
}

fn ffi(input: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    for i in 0..input.len() {
        let mut curr: i32 = 0;
        for (j, num) in input.iter().enumerate() {
            let pattern_val = get_pattern_val(i, j);
            curr += (*num as i32) * pattern_val;
        }
        res.push(curr.abs() % 10);
    }
    res
}

fn simulate(input: Vec<i32>, steps: i32) -> Vec<i32> {
    let mut res = input.clone();
    for _ in 0..steps {
        res = ffi(res);
    }
    res
}

pub fn part_a(input: &str, steps: i32) -> String {
    let input = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>();
    let output = simulate(input, steps);
    let mut res = 0;
    for i in 0..8 {
        res += output[i] * 10i32.pow(7 - i as u32);
    }
    res.to_string()
}

pub fn part_b(input: &str, steps: i32) -> String {
    let inp: Vec<i64> = input
        .chars()
        .filter(|line| line != &'\n')
        .map(|line| line.to_string().parse::<i64>().expect("Invalid number"))
        .collect();
    let offset = inp
        .iter()
        .take(7)
        .map(|v| v.to_string())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let mut cur = Vec::with_capacity(inp.len() * 10000);

    for _ in 0..10000 {
        for j in 0..inp.len() {
            cur.push(inp[j]);
        }
    }

    let mut cur = cur[offset..].to_vec();
    for _ in 0..steps {
        let mut suffix_sum = 0;
        for i in (0..cur.len()).rev() {
            suffix_sum = (suffix_sum + cur[i]) % 10;
            cur[i] = suffix_sum;
        }
    }
    cur.iter()
        .take(8)
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input, 100), "44098263");
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input, 100), "12482168");
    }
}
