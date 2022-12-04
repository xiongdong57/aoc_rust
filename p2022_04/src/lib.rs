fn parse_line(line: &str) -> (usize, usize, usize, usize) {
    let (pa, pb) = line.split_once(",").unwrap();
    let (a, b) = pa.split_once("-").unwrap();
    let (c, d) = pb.split_once("-").unwrap();
    (
        a.parse::<usize>().unwrap(),
        b.parse::<usize>().unwrap(),
        c.parse::<usize>().unwrap(),
        d.parse::<usize>().unwrap(),
    )
}

fn overlap(a: &usize, b: &usize, c: &usize, d: &usize) -> bool {
    assert!(a <= b);
    assert!(c <= d);
    (a <= c && b >= d) || (a >= c && b <= d)
}

pub fn part_a(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| parse_line(line))
        .filter(|(a, b, c, d)| overlap(a, b, c, d))
        .count()
}

fn overlap_v2(a: &usize, b: &usize, c: &usize, d: &usize) -> bool {
    assert!(a <= b);
    assert!(c <= d);
    (a <= c && c <= b) || (a <= d && d <= b) || (c <= a && b <= d)
}

pub fn part_b(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| parse_line(line))
        .filter(|(a, b, c, d)| overlap_v2(a, b, c, d))
        .count()
    // unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 2);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 584);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 4);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 933);
    }
}
