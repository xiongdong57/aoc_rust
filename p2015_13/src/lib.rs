use std::{collections::HashMap, collections::HashSet};

pub fn part_a(input: &str) -> i64 {
    let mut happiness = HashMap::new();
    let mut peoples = HashSet::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let from = parts[0];
        let symbol = parts[2];
        let num = parts[3].parse::<i64>().unwrap();
        let to = &parts[10][..parts[10].len() - 1];
        // remove the last character
        // *to.pop();
        match symbol {
            "gain" => {
                happiness.insert((from, to), num);
            }
            "lose" => {
                happiness.insert((from, to), -num);
            }
            _ => panic!("Unknown symbol: {}", symbol),
        }
        peoples.insert(from);
        peoples.insert(to);
    }

    // paths through all people for once
    let mut all_paths = Vec::new();
    let length = peoples.len();
    for person in &peoples {
        let mut paths = vec![vec![person]];
        while let Some(sub) = paths.pop() {
            if sub.len() == length {
                all_paths.push(sub);
            } else {
                for next in &peoples {
                    if !sub.contains(&next) {
                        let mut new_path = sub.clone();
                        new_path.push(next);
                        paths.push(new_path);
                    }
                }
            }
        }
    }

    let mut max_happiness = std::i64::MIN;
    for path in all_paths {
        let mut num = 0;
        for i in 0..path.len() {
            let from = path[i];
            let to = path[(i + 1) % path.len()];
            num += happiness.get(&(from, to)).unwrap();
            num += happiness.get(&(to, from)).unwrap();
        }
        max_happiness = max_happiness.max(num);
    }

    max_happiness
}

pub fn part_b(input: &str) -> i64 {
    let mut happiness = HashMap::new();
    let mut peoples = HashSet::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let from = parts[0];
        let symbol = parts[2];
        let num = parts[3].parse::<i64>().unwrap();
        let to = &parts[10][..parts[10].len() - 1];
        // remove the last character
        // *to.pop();
        match symbol {
            "gain" => {
                happiness.insert((from, to), num);
            }
            "lose" => {
                happiness.insert((from, to), -num);
            }
            _ => panic!("Unknown symbol: {}", symbol),
        }
        peoples.insert(from);
        peoples.insert(to);
    }

    //  add me to the list
    let me = "me";
    for people in &peoples {
        happiness.insert((people, me), 0);
        happiness.insert((me, people), 0);
    }

    peoples.insert(me);

    // paths through all people for once
    let mut all_paths = Vec::new();
    let length = peoples.len();
    for person in &peoples {
        let mut paths = vec![vec![person]];
        while let Some(sub) = paths.pop() {
            if sub.len() == length {
                all_paths.push(sub);
            } else {
                for next in &peoples {
                    if !sub.contains(&next) {
                        let mut new_path = sub.clone();
                        new_path.push(next);
                        paths.push(new_path);
                    }
                }
            }
        }
    }

    let mut max_happiness = std::i64::MIN;
    for path in all_paths {
        let mut num = 0;
        for i in 0..path.len() {
            let from = path[i];
            let to = path[(i + 1) % path.len()];
            num += happiness.get(&(from, to)).unwrap();
            num += happiness.get(&(to, from)).unwrap();
        }
        max_happiness = max_happiness.max(num);
    }

    max_happiness
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 330);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 709);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 668);
    }
}
