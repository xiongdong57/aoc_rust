use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Num(i32),
    List(Vec<Packet>),
}

fn parse(s: &str) -> Packet {
    if &s[0..1] == "[" {
        let mut stack: i32 = 0;
        Packet::List(
            s[1..s.len() - 1]
                .split(|c| {
                    if c == '[' {
                        stack += 1
                    } else if c == ']' {
                        stack -= 1
                    }
                    c == ',' && stack == 0
                })
                .filter_map(|s| (!s.is_empty()).then(|| parse(s)))
                .collect(),
        )
    } else {
        Packet::Num(s.parse().unwrap())
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Num(a), Packet::Num(b)) => a.cmp(b),
            (Packet::Num(_), _) => Packet::List(vec![self.clone()]).cmp(other),
            (_, Packet::Num(_)) => self.cmp(&Packet::List(vec![other.clone()])),
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
pub fn part_a(input: &str) -> usize {
    let packets: Vec<Packet> = input
        .trim()
        .lines()
        .filter_map(|s| (!s.is_empty()).then(|| parse(s)))
        .collect();

    let mut sum: usize = 0;
    for (i, s) in packets.chunks(2).enumerate() {
        let o = s[0].cmp(&s[1]);
        assert!(o != Ordering::Equal);
        if o == Ordering::Less {
            sum += i + 1;
        }
    }
    sum
}

pub fn part_b(input: &str) -> usize {
    let mut packets: Vec<Packet> = input
        .trim()
        .lines()
        .filter_map(|s| (!s.is_empty()).then(|| parse(s)))
        .collect();
    let d1 = parse("[[2]]");
    let d2 = parse("[[6]]");
    packets.push(d1.clone());
    packets.push(d2.clone());
    packets.sort();
    
    let loc1 = packets.binary_search(&d1).unwrap();
    let loc2 = packets.binary_search(&d2).unwrap();
    (loc1 + 1) * (loc2 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 13);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 5623);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 140);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 20570);
    }
}
