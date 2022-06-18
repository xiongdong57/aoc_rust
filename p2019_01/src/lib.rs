fn fuel(mass:i64) -> i64 {
    mass / 3 - 2
}

pub fn part_a(input: &str) -> i64 {
    input.lines().map(|line| fuel(line.parse::<i64>().unwrap())).sum()
}

fn fuel_v2(mass:i64) -> i64 {
    let fuel = mass / 3 - 2;
    match fuel {
        i if i <= 0 => 0,
        _ => fuel + fuel_v2(fuel),
    }
}

pub fn part_b(input: &str) -> i64 {
    input.lines().map(|line| fuel_v2(line.parse::<i64>().unwrap())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 3348430);
    }

    #[test]
    fn part_b_works() {
        assert_eq!(fuel_v2(14), 2);
        assert_eq!(fuel_v2(1969), 966);
        assert_eq!(fuel_v2(100756), 50346);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 5019767);
    }
}
