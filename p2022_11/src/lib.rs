use std::collections::HashMap;
use std::collections::VecDeque;

struct Operate {
    symbol: String,
    follow: String,
}

impl Operate {
    fn calc(&self, num: usize) -> usize {
        if self.symbol == "*" {
            if self.follow == "old" {
                return num * num;
            } else {
                return num * self.follow.parse::<usize>().unwrap();
            }
        } else if self.symbol == "+" {
            if self.follow == "old" {
                return num + num;
            } else {
                return num + self.follow.parse::<usize>().unwrap();
            }
        } else {
            panic!("no such symbol");
        }
    }
}

pub fn part_a(input: &str, cnt: usize) -> usize {
    let mut items = HashMap::new();
    let mut operations = HashMap::new();
    let mut divisions = HashMap::new();
    let mut true_to = HashMap::new();
    let mut false_to = HashMap::new();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut i = 0;
    let mut cur = 0;
    loop {
        if i + 1 > lines.len() {
            break;
        }
        if lines[i].len() == 0 {
            i = i + 1;
            continue;
        }
        let (pa, pb) = lines[i].split_once(":").unwrap();
        let (paa, pab) = pa.split_once(" ").unwrap();
        if paa == "Monkey" {
            cur = pab.parse::<usize>().unwrap();
        }
        if pa == "  Starting items" {
            items.insert(
                cur,
                pb.trim()
                    .split(", ")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<VecDeque<usize>>(),
            );
        }
        if pa == "  Operation" {
            let ch = pb.trim().chars().nth(10).unwrap();
            operations.insert(
                cur,
                Operate {
                    symbol: ch.to_string(),
                    follow: pb.trim().split(" ").last().unwrap().to_string(),
                },
            );
        }
        if pa == "  Test" {
            divisions.insert(
                cur,
                pb.trim()
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            );
        }
        if pa == "    If true" {
            true_to.insert(
                cur,
                pb.trim()
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            );
        }

        if pa == "    If false" {
            false_to.insert(
                cur,
                pb.trim()
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            );
        }
        i += 1;
    }

    // simulation
    let mut inspects: HashMap<usize, usize> = HashMap::new();
    for _ in 0..20 {
        for i in 0..cnt {
            loop {
                if items.get(&i).unwrap().len() == 0 {
                    break;
                }
                *inspects.entry(i).or_default() += 1;
                let mut item = items.get_mut(&i).unwrap().pop_front().unwrap();
                // operation
                item = operations.get(&i).unwrap().calc(item);
                // divide 3, round down
                item = item / 3;
                let div = divisions.get(&i).unwrap();
                if item % div == 0 {
                    let to = true_to.get(&i).unwrap();
                    items.get_mut(to).unwrap().push_back(item);
                } else {
                    let to = false_to.get(&i).unwrap();
                    items.get_mut(to).unwrap().push_back(item);
                }
            }
        }
    }
    let mut ins = inspects.into_values().collect::<Vec<usize>>();
    ins.sort();
    ins[cnt - 1] * ins[cnt - 2]
}

pub fn part_b(input: &str, cnt: usize) -> usize {
    let mut items = HashMap::new();
    let mut operations = HashMap::new();
    let mut divisions = HashMap::new();
    let mut true_to = HashMap::new();
    let mut false_to = HashMap::new();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut i = 0;
    let mut cur = 0;
    loop {
        if i + 1 > lines.len() {
            break;
        }
        if lines[i].len() == 0 {
            i = i + 1;
            continue;
        }
        let (pa, pb) = lines[i].split_once(":").unwrap();
        let (paa, pab) = pa.split_once(" ").unwrap();
        if paa == "Monkey" {
            cur = pab.parse::<usize>().unwrap();
        }
        if pa == "  Starting items" {
            items.insert(
                cur,
                pb.trim()
                    .split(", ")
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<VecDeque<usize>>(),
            );
        }
        if pa == "  Operation" {
            let ch = pb.trim().chars().nth(10).unwrap();
            operations.insert(
                cur,
                Operate {
                    symbol: ch.to_string(),
                    follow: pb.trim().split(" ").last().unwrap().to_string(),
                },
            );
        }
        if pa == "  Test" {
            divisions.insert(
                cur,
                pb.trim()
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            );
        }
        if pa == "    If true" {
            true_to.insert(
                cur,
                pb.trim()
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            );
        }

        if pa == "    If false" {
            false_to.insert(
                cur,
                pb.trim()
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap(),
            );
        }
        i += 1;
    }

    // simulation
    let mut inspects: HashMap<usize, usize> = HashMap::new();
    let modular = divisions.values().into_iter().fold(1, |acc, x| acc * x);
    for _ in 0..10000 {
        for i in 0..cnt {
            loop {
                if items.get(&i).unwrap().len() == 0 {
                    break;
                }
                *inspects.entry(i).or_default() += 1;
                let mut item = items.get_mut(&i).unwrap().pop_front().unwrap();
                // operation
                item = operations.get(&i).unwrap().calc(item);
                // https://www.reddit.com/r/adventofcode/comments/zih7gf/2022_day_11_part_2_what_does_it_mean_find_another
                // https://libraryguides.centennialcollege.ca/c.php?g=717548&p=5121840
                item = item % modular;
                let div = divisions.get(&i).unwrap();
                if item % div == 0 {
                    let to = true_to.get(&i).unwrap();
                    items.get_mut(to).unwrap().push_back(item);
                } else {
                    let to = false_to.get(&i).unwrap();
                    items.get_mut(to).unwrap().push_back(item);
                }
            }
        }
    }
    let mut ins = inspects.into_values().collect::<Vec<usize>>();
    ins.sort();
    ins[cnt - 1] * ins[cnt - 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input, 4), 10605);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input, 8), 55930);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input, 4), 2713310158);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input, 8), 14636993466);
    }
}
