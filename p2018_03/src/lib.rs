use serde_scan;
use std::collections::{HashMap, HashSet};

pub fn part_a(input: &str) -> i64 {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (id, x, y, w, h): (u32, u32, u32, u32, u32) =
            serde_scan::scan!("#{} @ {},{}: {}x{}" <- line).unwrap();
        for i in x..(x + w) {
            for j in y..(y + h) {
                let key = (i, j);
                map.entry(key).or_insert(HashSet::new()).insert(id);
            }
        }
    }
    map.values().filter(|v| v.len() > 1).count() as i64
}

pub fn part_b(input: &str) -> u32 {
    let mut map = HashMap::new();
    let mut nodes = HashSet::new();
    for line in input.lines() {
        let (id, x, y, w, h): (u32, u32, u32, u32, u32) =
            serde_scan::scan!("#{} @ {},{}: {}x{}" <- line).unwrap();
        nodes.insert(id);
        for i in x..(x + w) {
            for j in y..(y + h) {
                let key = (i, j);
                map.entry(key).or_insert(HashSet::new()).insert(id);
            }
        }
    }
    for node in nodes {
        if unique(&map, node) {
            return node;
        }
    }
    panic!("No unique nodes found");
}

fn unique(map: &HashMap<(u32, u32), HashSet<u32>>, node: u32) -> bool {
    for val in map.values() {
        if val.len() > 1 && val.contains(&node) {
            return false;
        }
    }
    true
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
        assert_eq!(part_a(input), 112418);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 3);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 560);
    }
}
