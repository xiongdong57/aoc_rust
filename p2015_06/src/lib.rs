use regex::Regex;
use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let re =
        Regex::new(r"^(?P<action>.*?) (?P<x0>\d+),(?P<y0>\d+) through (?P<x1>\d+),(?P<y1>\d+)$")
            .unwrap();
    let mut lights = HashMap::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let action = caps.name("action").unwrap().as_str();
        let (x0, y0, x1, y1) = (
            caps.name("x0").unwrap().as_str().parse::<i64>().unwrap(),
            caps.name("y0").unwrap().as_str().parse::<i64>().unwrap(),
            caps.name("x1").unwrap().as_str().parse::<i64>().unwrap(),
            caps.name("y1").unwrap().as_str().parse::<i64>().unwrap(),
        );

        match action {
            "turn on" => {
                for x in x0.min(x1)..=x0.max(x1) {
                    for y in y0.min(y1)..=y0.max(y1) {
                        *lights.entry((x, y)).or_insert(0) = 1;
                    }
                }
            }
            "turn off" => {
                for x in x0.min(x1)..=x0.max(x1) {
                    for y in y0.min(y1)..=y0.max(y1) {
                        *lights.entry((x, y)).or_insert(0) = 0;
                    }
                }
            }
            "toggle" => {
                for x in x0.min(x1)..=x0.max(x1) {
                    for y in y0.min(y1)..=y0.max(y1) {
                        *lights.entry((x, y)).or_insert(0) = 1 - *lights.entry((x, y)).or_insert(0);
                    }
                }
            }
            _ => panic!("Unknown action: {}", action),
        }
    }
    lights.values().sum()
}

pub fn part_b(input: &str) -> i64 {
    let re =
        Regex::new(r"^(?P<action>.*?) (?P<x0>\d+),(?P<y0>\d+) through (?P<x1>\d+),(?P<y1>\d+)$")
            .unwrap();
    let mut lights = HashMap::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let action = caps.name("action").unwrap().as_str();
        let (x0, y0, x1, y1) = (
            caps.name("x0").unwrap().as_str().parse::<i64>().unwrap(),
            caps.name("y0").unwrap().as_str().parse::<i64>().unwrap(),
            caps.name("x1").unwrap().as_str().parse::<i64>().unwrap(),
            caps.name("y1").unwrap().as_str().parse::<i64>().unwrap(),
        );

        let light_changes: i64;
        match action {
            "turn on" => {
                light_changes = 1;
            }
            "turn off" => {
                light_changes = -1;
            }
            "toggle" => {
                light_changes = 2;
            }
            _ => panic!("Unknown action: {}", action),
        }
        for x in x0.min(x1)..=x0.max(x1) {
            for y in y0.min(y1)..=y0.max(y1) {
                // when light_changes is -1, the minimum of light is 0
                *lights.entry((x, y)).or_insert(0) =
                    0.max(*lights.entry((x, y)).or_insert(0) + light_changes);
            }
        }
    }
    lights.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = "toggle 0,0 through 999,0";
        assert_eq!(part_a(input), 1000);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 543903);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 14687245);
    }
}
