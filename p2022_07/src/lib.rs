use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct FileDir {
    name: String,
    size: Option<i64>,
}

impl FromStr for FileDir {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((p1, p2)) = s.split_once(" ") {
            if p1 == "dir" {
                Ok(FileDir {
                    name: p2.to_string(),
                    size: None,
                })
            } else {
                Ok(FileDir {
                    name: p2.to_string(),
                    size: Some(p1.parse::<i64>()?),
                })
            }
        } else {
            panic!("parse err")
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let mut tree = HashMap::new();
    let mut cur = "".to_string();
    let mut prev: Vec<String> = vec![];
    for line in input.lines() {
        let p = line.split(" ").collect::<Vec<&str>>();
        if p[0] == "$" {
            if p[1] == "cd" {
                if p[2] == ".." {
                    prev.pop();
                } else {
                    let cur = if let Some(c) = prev.last() {
                        format!("{}/{}", c, p[2])
                    } else {
                        format!("{}/{}", cur, p[2])
                    };
                    prev.push(cur.clone());
                    tree.insert(cur, vec![]);
                }
            } else if p[1] == "ls" {
                cur = prev.last().unwrap().to_string();
            } else {
                panic!("no such command: {}", p[1]);
            }
        } else {
            let d = FileDir::from_str(line).unwrap();
            tree.get_mut(&cur).unwrap().push(d);
        }
    }

    tree.keys()
        .into_iter()
        .map(|n| get_size(n, &tree))
        .filter(|size| size <= &100000)
        .sum()
}

fn get_size(name: &str, tree: &HashMap<String, Vec<FileDir>>) -> i64 {
    let mut size = 0;
    if tree.contains_key(name) {
        for node in &tree[name] {
            match node.size {
                Some(s) => size += s,
                None => {
                    let name = format!("{}/{}", name, node.name);
                    size += get_size(&name, tree)
                }
            }
        }
    }
    size
}

pub fn part_b(input: &str) -> i64 {
    let mut tree = HashMap::new();
    let mut cur = "".to_string();
    let mut prev: Vec<String> = vec![];
    for line in input.lines() {
        let p = line.split(" ").collect::<Vec<&str>>();
        if p[0] == "$" {
            if p[1] == "cd" {
                if p[2] == ".." {
                    prev.pop();
                } else {
                    let cur = if let Some(c) = prev.last() {
                        format!("{}/{}", c, p[2])
                    } else {
                        format!("{}/{}", cur, p[2])
                    };
                    prev.push(cur.clone());
                    tree.insert(cur, vec![]);
                }
            } else if p[1] == "ls" {
                cur = prev.last().unwrap().to_string();
            } else {
                panic!("no such command: {}", p[1]);
            }
        } else {
            let d = FileDir::from_str(line).unwrap();
            tree.get_mut(&cur).unwrap().push(d);
        }
    }

    let mut sizes: Vec<i64> = tree
        .keys()
        .into_iter()
        .map(|n| get_size(n, &tree))
        .collect();
    sizes.sort();
    let unused = 70000000 - get_size("//", &tree);
    for s in sizes {
        if s + unused >= 30000000 {
            return s;
        }
    }
    panic!("no solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 95437);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 1444896);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 24933642);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 404395);
    }
}
