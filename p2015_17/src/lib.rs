fn combination(candidate: &[i64], remaining: i64) -> i64 {
    match remaining {
        0 => 1,
        i if i < 0 => 0,
        _ => {
            let mut count = 0;
            for i in 0..candidate.len() {
                count += combination(&candidate[i + 1..], remaining - candidate[i]);
            }
            count
        }
    }
}

pub fn part_a(input: &str, remaining: i64) -> i64 {
    let condidate: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    combination(&condidate, remaining)
}

fn combination_v2(candidate: &[i64], remaining: i64, containers: Vec<i64>) -> Vec<Vec<i64>> {
    match remaining {
        0 => return vec![containers],
        i if i < 0 => return vec![vec![]],
        _ => {
            let mut combinations = Vec::new();
            for i in 0..candidate.len() {
                let mut new_containers = containers.clone();
                new_containers.push(candidate[i]);
                let cur_combination = combination_v2(
                    &candidate[i + 1..],
                    remaining - candidate[i],
                    new_containers,
                );
                if cur_combination.len() > 0 && cur_combination[0].len() > 0 {
                    combinations.extend(cur_combination);
                }
            }
            combinations
        }
    }
}

pub fn part_b(input: &str, remaining: i64) -> i64 {
    let condidate: Vec<i64> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
    let combinations = combination_v2(&condidate, remaining, vec![]);
    let min_count = combinations.iter().min_by_key(|x| x.len()).unwrap().len();

    combinations.iter().filter(|x| x.len() == min_count).count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = "20\n15\n10\n5\n5";
        assert_eq!(part_a(input, 25), 4);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input, 150), 654);
    }

    #[test]
    fn part_b_works() {
        let input = "20\n15\n10\n5\n5";
        assert_eq!(part_b(input, 25), 3);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input, 150), 57);
    }
}
