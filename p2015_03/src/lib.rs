use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let (mut x, mut y): (i32, i32) = (0, 0);
    map.insert((x, y), 1);
    for ch in input.chars() {
        match ch {
            '^' => y += 1,
            '>' => x += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            _ => panic!("Unknown character: {}", ch),
        }
        let count = map.entry((x, y)).or_insert(1);
        *count += 1;
    }
    map.len() as i64
}

pub fn part_b(input: &str) -> i64 {
    let mut santa_map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut robo_map: HashMap<(i32, i32), i32> = HashMap::new();
    let (mut santa_x, mut santa_y) = (0, 0);
    let (mut robo_x, mut robo_y) = (0, 0);
    santa_map.insert((santa_x, santa_y), 1);
    robo_map.insert((robo_x, robo_y), 1);
    for (i, ch) in input.chars().enumerate() {
        if i % 2 == 0 {
            match ch {
                '^' => santa_y += 1,
                '>' => santa_x += 1,
                'v' => santa_y -= 1,
                '<' => santa_x -= 1,
                _ => panic!("Unknown character: {}", ch),
            }
            let count = santa_map.entry((santa_x, santa_y)).or_insert(1);
            *count += 1;
        } else {
            match ch {
                '^' => robo_y += 1,
                '>' => robo_x += 1,
                'v' => robo_y -= 1,
                '<' => robo_x -= 1,
                _ => panic!("Unknown character: {}", ch),
            }
            let count = robo_map.entry((robo_x, robo_y)).or_insert(1);
            *count += 1;
        }
    }

    // count two maps key, only unique keys
    let mut count = santa_map.len();
    for key in robo_map.keys() {
        if !santa_map.contains_key(key) {
            count += 1;
        }
    }
    count as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = ">";
        assert_eq!(part_a(input), 2);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 2081);
    }

    #[test]
    fn part_b_works() {
        let input = "^v^v^v^v^v";
        assert_eq!(part_b(input), 11);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 2341);
    }
}
