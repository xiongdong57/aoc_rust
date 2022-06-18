use std::collections::{BTreeMap, BTreeSet};

fn count_path(path: &BTreeMap<&str, &str>, node: &str) -> usize {
    match node {
        "COM" => 0,
        _ => 1 + count_path(path, path[node]),
    }
}

pub fn part_a(input: &str) -> usize {
    let mut path = BTreeMap::new();
    let mut obs = BTreeSet::new();
    for line in input.lines() {
        let (from, to) = line.split_once(")").unwrap();
        obs.insert(from);
        obs.insert(to);
        path.insert(to, from);
    }

    obs.iter().map(|x| count_path(&path, x)).sum()
}

fn gen_path<'a>(path: &BTreeMap<&'a str, &'a str>, node: &'a str) -> Vec<&'a str> {
    match node {
        "COM" => vec!["COM"],
        _ => vec![node]
            .iter()
            .chain(gen_path(path, path[node]).iter())
            .cloned()
            .collect(),
    }
}

pub fn part_b(input: &str) -> i64 {
    let mut path = BTreeMap::new();
    for line in input.lines() {
        let (from, to) = line.split_once(")").unwrap();
        path.insert(to, from);
    }

    let path1 = gen_path(&path, "YOU");
    let path2 = gen_path(&path, "SAN");
    for (i, node) in path1.iter().enumerate() {
        for (j, node2) in path2.iter().enumerate() {
            if node == node2 {
                return i as i64 + j as i64 - 2;
            }
        }
    }
    panic!("No common node found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 42);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 300598);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test2.txt");
        assert_eq!(part_b(input), 4);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 520);
    }
}
