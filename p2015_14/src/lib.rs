struct Player {
    speed: i64,
    fly_time: i64,
    rest_time: i64,
}

fn parse_input(input: &str) -> Vec<Player> {
    let mut players = Vec::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        players.push(Player {
            speed: parts[3].parse::<i64>().unwrap(),
            fly_time: parts[6].parse::<i64>().unwrap(),
            rest_time: parts[13].parse::<i64>().unwrap(),
        });
    }
    players
}

pub fn part_a(input: &str, duration: i64) -> i64 {
    let players = parse_input(input);

    let mut max_distance = i64::MIN;
    for player in players {
        let total_fly_time = duration / (player.fly_time + player.rest_time) * player.fly_time
            + (duration % (player.fly_time + player.rest_time)).min(player.fly_time);
        let distance = player.speed * total_fly_time;
        max_distance = max_distance.max(distance);
    }
    max_distance
}

pub fn part_b(input: &str, duration: i64) -> i64 {
    let players = parse_input(input);

    let mut points = Vec::new();
    for _ in &players {
        points.push(0);
    }
    for t in 1..=duration {
        let mut distances = Vec::new();
        for player in &players {
            let total_fly_time = t / (player.fly_time + player.rest_time) * player.fly_time
                + (t % (player.fly_time + player.rest_time)).min(player.fly_time);
            let distance = player.speed * total_fly_time;
            distances.push(distance);
        }

        let max_dis = distances.iter().max().unwrap();
        for (i, dis) in distances.iter().enumerate() {
            if dis == max_dis {
                points[i] += 1;
            }
        }
    }
    *points.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input, 1000), 1120);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input, 2503), 2640);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input, 1000), 689);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input, 2503), 1102);
    }
}
