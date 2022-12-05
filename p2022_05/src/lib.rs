use std::collections::{HashMap, VecDeque};

pub fn part_a(input: &str, cnt: usize) -> String {
    let mut map: HashMap<usize, VecDeque<char>> = HashMap::new();
    let (pa, pb) = input.split_once("\r\n\r\n").unwrap();
    for line in pa.lines() {
        for i in 1..=cnt {
            let p = line.chars().nth(4 * i - 3).unwrap();
            if 'A' <= p && p <= 'Z' {
                map.entry(i).or_default().push_front(p)
            }
        }
    }
    for step in pb.lines() {
        let ps = step.split(" ").collect::<Vec<&str>>();
        let c = ps[1].parse::<i32>().unwrap();
        let f = ps[3].parse::<usize>().unwrap();
        let t = ps[5].parse::<usize>().unwrap();
        for _ in 0..c {
            let elem = map.get_mut(&f).unwrap().pop_back().unwrap();
            map.get_mut(&t).unwrap().push_back(elem);
        }
    }
    let mut tops = vec![];
    for i in 1..=cnt {
        tops.push(map.get_mut(&i).unwrap().pop_back().unwrap().to_string());
    }
    tops.join("")
}

pub fn part_b(input: &str, cnt: usize) -> String {
    let mut map: HashMap<usize, VecDeque<char>> = HashMap::new();
    let (pa, pb) = input.split_once("\r\n\r\n").unwrap();
    for line in pa.lines() {
        for i in 1..=cnt {
            let p = line.chars().nth(4 * i - 3).unwrap();
            if 'A' <= p && p <= 'Z' {
                map.entry(i).or_default().push_front(p)
            }
        }
    }
    for step in pb.lines() {
        let ps = step.split(" ").collect::<Vec<&str>>();
        let c = ps[1].parse::<i32>().unwrap();
        let f = ps[3].parse::<usize>().unwrap();
        let t = ps[5].parse::<usize>().unwrap();
        let mut elems = vec![];
        for _ in 0..c {
            let elem = map.get_mut(&f).unwrap().pop_back().unwrap();
            elems.push(elem);
        }
        for i in (0..c).rev() {
            let elem = elems[i as usize];
            map.get_mut(&t).unwrap().push_back(elem);
        }
    }
    let mut tops = vec![];
    for i in 1..=cnt {
        tops.push(map.get_mut(&i).unwrap().pop_back().unwrap().to_string());
    }
    tops.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input, 3), "CMZ");
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input, 9), "BZLVHBWQF");
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input, 3), "MCD");
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input, 9), "TDGJQTZSL");
    }
}
