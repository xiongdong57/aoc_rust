struct Item {
    value: i64,
    origin_idx: usize,
}

fn parse(input: &str) -> Vec<Item> {
    let mut items = Vec::new();
    for (i, line) in input.trim().lines().enumerate() {
        items.push(Item {
            value: line.parse::<i64>().unwrap(),
            origin_idx: i,
        });
    }
    items
}

fn mix_item(items: &mut Vec<Item>, origin_idx: usize) {
    let current_idx = items
        .iter()
        .position(|r| r.origin_idx == origin_idx)
        .unwrap();
    let value = items[current_idx].value;
    if value == 0 {
        // not move
        return;
    }

    let swap_mod = (items.len() - 1) as i64;
    if value > 0 {
        let swap_steps = (value % swap_mod) as usize;
        for i in 0..swap_steps {
            let idx0 = (current_idx + i) % items.len();
            let idx1 = (current_idx + i + 1) % items.len();
            items.swap(idx0, idx1);
        }
    } else {
        let swap_steps = (-value % swap_mod) as usize;
        for i in 0..swap_steps {
            let idx0 = (current_idx + items.len() * 2 - i) % items.len();
            let idx1 = (current_idx + items.len() * 2 - i - 1) % items.len();
            items.swap(idx0, idx1);
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let mut items = parse(input);
    for i in 0..items.len() {
        mix_item(&mut items, i);
    }

    let zero_idx = items.iter().position(|x| x.value == 0).unwrap();
    let value1000 = items[(zero_idx + 1000) % items.len()].value;
    let value2000 = items[(zero_idx + 2000) % items.len()].value;
    let value3000 = items[(zero_idx + 3000) % items.len()].value;

    value1000 + value2000 + value3000
}

pub fn part_b(input: &str) -> i64 {
    let mut items = parse(input);
    for item in items.iter_mut() {
        item.value *= 811589153;
    }
    for _ in 0..10 {
        for i in 0..items.len() {
            mix_item(&mut items, i);
        }
    }

    let zero_idx = items.iter().position(|x| x.value == 0).unwrap();
    let value1000 = items[(zero_idx + 1000) % items.len()].value;
    let value2000 = items[(zero_idx + 2000) % items.len()].value;
    let value3000 = items[(zero_idx + 3000) % items.len()].value;

    value1000 + value2000 + value3000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 3);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 8372);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 1623178306);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 7865110481723);
    }
}
