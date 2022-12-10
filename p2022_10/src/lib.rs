pub fn part_a(input: &str) -> i64 {
    let mut steps = Vec::new();
    for line in input.lines() {
        if let Some((_, pb)) = line.split_once(" ") {
            steps.push(0);
            steps.push(pb.parse::<i64>().unwrap())
        } else {
            steps.push(0);
        }
    }

    let mut score = 0;
    for n in vec![20, 60, 100, 140, 180, 220] {
        let tmp = 1 + steps.iter().cycle().take(n - 1).sum::<i64>();
        score += tmp * (n as i64);
    }
    score
}

pub fn part_b(input: &str) {
    let mut steps = Vec::new();
    for line in input.lines() {
        if let Some((_, pb)) = line.split_once(" ") {
            steps.push(0);
            steps.push(pb.parse::<i64>().unwrap())
        } else {
            steps.push(0);
        }
    }
    let mut r = 1;
    let mut cycle = 0;
    for s in steps {
        draw(cycle, r);
        cycle += 1;
        r += s;
    }
}

fn draw(cycle: i64, r: i64) {
    let x = cycle % 40;
    if (r - x).abs() <= 1 {
        print!("#");
    } else {
        print!(".");
    }

    if x == 39 {
        print!("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 13140);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 15020);
    }

    // #[test]
    // fn part_b_works() {
    //     let input = include_str!("test.txt");
    //     assert_eq!(part_b(input), ());
    // }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), ());
    }
}
