use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Ops {
    left: String,
    right: String,
    op: String,
}

enum Yell {
    Ops(Ops),
    Num(i64),
}

fn parse(input: &str) -> HashMap<String, Yell> {
    let mut map = HashMap::new();
    for line in input.trim().lines() {
        let (l, r) = line.split_once(": ").unwrap();
        let rs = r.split(" ").collect::<Vec<&str>>();
        if rs.len() == 1 {
            map.insert(l.to_string(), Yell::Num(r.parse::<i64>().unwrap()));
        } else {
            map.insert(
                l.to_string(),
                Yell::Ops(Ops {
                    left: rs[0].to_string(),
                    right: rs[2].to_string(),
                    op: rs[1].to_string(),
                }),
            );
        }
    }
    map
}

fn eval(map: &HashMap<String, Yell>, node: &str) -> i64 {
    if !map.contains_key(node) {
        panic!("wrong keys")
    } else {
        match &map[node] {
            Yell::Num(num) => *num,
            Yell::Ops(ops) => {
                let l = eval(map, &ops.left);
                let r = eval(map, &ops.right);
                if ops.op == "+" {
                    l + r
                } else if ops.op == "-" {
                    l - r
                } else if ops.op == "*" {
                    l * r
                } else if ops.op == "/" {
                    l / r
                } else {
                    panic!("wrong op")
                }
            }
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let map = parse(input);
    eval(&map, "root")
}

pub fn part_b(input: &str) -> i64 {
    let mut map = parse(input);
    if let Yell::Ops(ops) = &map["root"] {
        *map.get_mut("root").unwrap() = Yell::Ops(Ops {
            left: ops.left.to_string(),
            right: ops.right.to_string(),
            op: "-".to_string(),
        })
    }
    let mut lower_bound = 0;
    let mut upper_bound = 0;
    let mut human_val = 1;
    loop {
        *map.get_mut("humn").unwrap() = Yell::Num(human_val);
        let root_val = eval(&map, "root");
        if root_val == 0 {
            return human_val;
        }

        if root_val < 0 {
            lower_bound = human_val;
        } else {
            upper_bound = human_val;
        }

        if lower_bound == 0 || upper_bound == 0 {
            human_val *= 2;
        } else if lower_bound == upper_bound {
            panic!("no solution")
        } else {
            human_val = (upper_bound + lower_bound) / 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 152);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 268597611536314);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 302);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 3451534022348);
    }
}
