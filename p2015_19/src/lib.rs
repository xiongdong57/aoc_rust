use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> (HashMap<&str, Vec<&str>>, &str) {
    let mut mapping: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut words = input.split("\r\n\r\n");
    for word in words.next().unwrap().split("\r\n") {
        let mut parts = word.split(" => ");
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        mapping.entry(from).or_insert(Vec::new()).push(to);
    }

    let word = words.next().unwrap();

    (mapping, word)
}

fn simulate(mapping: &HashMap<&str, Vec<&str>>, word: &str) -> HashSet<String> {
    let mut collections = HashSet::new();
    for i in 0..word.len() {
        for (key, values) in mapping.iter() {
            if word[i..].starts_with(key) {
                for to in values {
                    let new_word = format!("{}{}{}", &word[0..i], to, &word[i + key.len()..]);
                    collections.insert(new_word);
                }
            }
        }
    }
    collections
}

pub fn part_a(input: &str) -> i64 {
    let (mapping, word) = parse_input(input);
    let collections = simulate(&mapping, word);

    collections.len() as i64
}

fn parse_input_v2(input: &str) -> (HashMap<&str, Vec<&str>>, &str) {
    let mut mapping: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut words = input.split("\r\n\r\n");
    for word in words.next().unwrap().split("\r\n") {
        let mut parts = word.split(" => ");
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        mapping.entry(to).or_insert(Vec::new()).push(from);
    }

    let word = words.next().unwrap();

    (mapping, word)
}

pub fn part_b(input: &str) -> i64 {
    let (mapping, goal) = parse_input_v2(input);
    let mut seen = BTreeSet::new();
    let mut to_visit: BinaryHeap<(Reverse<i64>, Reverse<i64>, String)> = BinaryHeap::new();
    to_visit.push((Reverse(1), Reverse(0), goal.to_string()));
    while let Some((_, iters, w)) = to_visit.pop() {
        if w == "e" {
            return iters.0;
        } else {
            for word in simulate(&mapping, &w) {
                if seen.insert(word.clone()) {
                    to_visit.push((Reverse(word.len() as i64), Reverse(iters.0 + 1), word));
                }
            }
        }
    }

    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 4);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 509);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test2.txt");
        assert_eq!(part_b(input), 6);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 195);
    }
}
