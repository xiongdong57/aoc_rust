use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let mut distances: HashMap<(&str, &str), i64> = HashMap::new();
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let from = parts[0];
        let to = parts[2];
        let distance = parts[4].parse::<i64>().unwrap();

        distances.insert((from, to), distance);
        distances.insert((to, from), distance);

        edges.entry(from).or_insert(Vec::new()).push(to);
        edges.entry(to).or_insert(Vec::new()).push(from);
    }

    // Find the path passing through each node exactly once.
    let nodes = edges.keys().cloned().collect::<Vec<_>>();
    let length = nodes.len();
    let mut all_paths = Vec::new();
    for node in nodes.into_iter() {
        let mut paths = vec![vec![node]];
        while let Some(sub) = paths.pop() {
            if sub.len() == length {
                all_paths.push(sub);
            } else {
                for next in &edges[sub[sub.len() - 1]] {
                    if !sub.contains(&next) {
                        let mut new_path = sub.clone();
                        new_path.push(&next);
                        paths.push(new_path);
                    }
                }
            }
        }
    }

    // Find the shortest path.
    let mut shortest_distance = std::i64::MAX;
    for path in all_paths {
        let mut distance = 0;
        for i in 0..path.len() - 1 {
            let from = path[i];
            let to = path[(i + 1)];
            distance += distances[&(from, to)];
        }

        shortest_distance = shortest_distance.min(distance)
    }
    shortest_distance
}

pub fn part_b(input: &str) -> i64 {
    let mut distances: HashMap<(&str, &str), i64> = HashMap::new();
    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let from = parts[0];
        let to = parts[2];
        let distance = parts[4].parse::<i64>().unwrap();

        distances.insert((from, to), distance);
        distances.insert((to, from), distance);

        edges.entry(from).or_insert(Vec::new()).push(to);
        edges.entry(to).or_insert(Vec::new()).push(from);
    }

    // Find the path passing through each node exactly once.
    let nodes = edges.keys().cloned().collect::<Vec<_>>();
    let length = nodes.len();
    let mut all_paths = Vec::new();
    for node in nodes.into_iter() {
        let mut paths = vec![vec![node]];
        while let Some(sub) = paths.pop() {
            if sub.len() == length {
                all_paths.push(sub);
            } else {
                for next in &edges[sub[sub.len() - 1]] {
                    if !sub.contains(&next) {
                        let mut new_path = sub.clone();
                        new_path.push(&next);
                        paths.push(new_path);
                    }
                }
            }
        }
    }

    // Find the longest path.
    let mut longest_distance = std::i64::MIN;
    for path in all_paths {
        let mut distance = 0;
        for i in 0..path.len() - 1 {
            let from = path[i];
            let to = path[(i + 1)];
            distance += distances[&(from, to)];
        }

        longest_distance = longest_distance.max(distance)
    }
    longest_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 605);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 141);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 982);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 736);
    }
}
