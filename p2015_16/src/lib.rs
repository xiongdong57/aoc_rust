use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<&str, HashMap<&str, i64>> {
    let mut peoples = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once(": ").unwrap();
        let name = left.split(" ").collect::<Vec<&str>>()[1];
        let subs: Vec<&str> = right.split(", ").collect();
        let mut people = HashMap::new();
        for sub in subs {
            let parts: Vec<&str> = sub.split(": ").collect();
            let sub_name = parts[0];
            let sub_count = parts[1].parse::<i64>().unwrap();
            people.insert(sub_name, sub_count);
        }
        peoples.insert(name, people);
    }
    peoples
}

pub fn part_a(input: &str) -> &str {
    let peoples = parse_input(input);
    let mut known = HashMap::new();
    known.insert("children", 3);
    known.insert("cats", 7);
    known.insert("samoyeds", 2);
    known.insert("pomeranians", 3);
    known.insert("akitas", 0);
    known.insert("vizslas", 0);
    known.insert("goldfish", 5);
    known.insert("trees", 3);
    known.insert("cars", 2);
    known.insert("perfumes", 1);

    for (name, p) in peoples {
        if p.iter()
            .all(|(k, v)| known.contains_key(k) && known.get(k).unwrap() == v)
        {
            return name;
        }
    }
    panic!("No match found");
}

pub fn part_b(input: &str) -> &str {
    let peoples = parse_input(input);
    let mut known = HashMap::new();
    known.insert("children", 3);
    known.insert("cats", 7);
    known.insert("samoyeds", 2);
    known.insert("pomeranians", 3);
    known.insert("akitas", 0);
    known.insert("vizslas", 0);
    known.insert("goldfish", 5);
    known.insert("trees", 3);
    known.insert("cars", 2);
    known.insert("perfumes", 1);

    for (name, p) in peoples {
        if p.iter().all(|(k, v)| {
            if known.contains_key(k) {
                if *k == "cats" || *k == "trees" {
                    v > known.get(k).unwrap()
                } else if *k == "pomeranians" || *k == "goldfish" {
                    v < known.get(k).unwrap()
                } else {
                    known.get(k).unwrap() == v
                }
            } else {
                true
            }
        }) {
            return name;
        }
    }
    panic!("No match found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), "373");
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), "260");
    }
}
