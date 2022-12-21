use std::collections::BTreeMap;

type Blueprint<'a> = BTreeMap<&'a str, BTreeMap<&'a str, usize>>;

struct State<'a> {
    robots: BTreeMap<&'a str, usize>,
    materials: BTreeMap<&'a str, usize>,
    time: usize,
}

fn parse(line: &str) -> Option<(usize, Blueprint)> {
    let line = line.strip_prefix("Blueprint ")?;
    let (id, line) = line.split_once(':')?;
    let id = id.parse().ok()?;
    let blueprint = line
        .split('.')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let part = part.strip_prefix(" Each ")?;
            let (robot, part) = part.split_once(' ')?;
            let part = part.strip_prefix("robot costs ")?;
            let costs = part
                .split(" and ")
                .map(|s| {
                    let (cost, material) = s.split_once(' ')?;
                    Some((material, cost.parse().ok()?))
                })
                .collect::<Option<_>>()?;
            Some((robot, costs))
        })
        .collect::<Option<_>>()?;
    Some((id, blueprint))
}

fn geodes(blueprint: &Blueprint, time: usize) -> usize {
    let mut max_values = BTreeMap::new();
    for costs in blueprint.values() {
        for (&material, &cost) in costs {
            max_values
                .entry(material)
                .and_modify(|e| *e = cost.max(*e))
                .or_insert(cost);
        }
    }
    let mut best = 0;
    let mut queue = vec![State {
        robots: [("ore", 1)].into(),
        materials: [].into(),
        time,
    }];
    while let Some(State {
        robots,
        materials,
        time,
    }) = queue.pop()
    {
        let potential = {
            let mut additional_robots = BTreeMap::new();
            let mut materials = materials.clone();
            for _ in 0..time {
                for (&robot, &count) in robots.iter().chain(additional_robots.iter()) {
                    materials
                        .entry(robot)
                        .and_modify(|e| *e += count)
                        .or_insert(count);
                }
                for robot in blueprint
                    .iter()
                    .filter(|(&robot, costs)| {
                        costs.iter().all(|(&material, &cost)| {
                            materials.get(material).copied().unwrap_or(0)
                                >= cost * (additional_robots.get(robot).unwrap_or(&0) + 1)
                        })
                    })
                    .map(|(&robot, _)| robot)
                    .collect::<Vec<_>>()
                {
                    additional_robots
                        .entry(robot)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
            }
            materials.get("geode").copied().unwrap_or(0)
        };
        if potential < best {
            continue;
        }
        let estimate =
            materials.get("geode").unwrap_or(&0) + robots.get("geode").unwrap_or(&0) * time;
        best = best.max(estimate);
        for (&robot, costs) in blueprint {
            if max_values
                .get(robot)
                .filter(|&max_value| robots.get(robot).unwrap_or(&0) >= max_value)
                .is_some()
            {
                continue;
            }
            let delta = costs
                .iter()
                .filter_map(|(&material, &cost)| -> Option<usize> {
                    let demand = cost.checked_sub(*materials.get(material).unwrap_or(&0))?;
                    let supply = robots.get(material).copied().unwrap_or(0);
                    if supply == 0 {
                        Some(usize::MAX)
                    } else {
                        Some((demand + supply - 1) / supply)
                    }
                })
                .max()
                .unwrap_or(0);
            if delta < time {
                let mut materials = materials.clone();
                for (&robot, &count) in &robots {
                    materials
                        .entry(robot)
                        .and_modify(|e| *e += count * (delta + 1))
                        .or_insert(count * (delta + 1));
                }
                for (&material, &cost) in costs {
                    *materials.get_mut(material).unwrap() -= cost;
                }
                let mut robots = robots.clone();
                robots.entry(robot).and_modify(|e| *e += 1).or_insert(1);
                queue.push(State {
                    robots,
                    materials,
                    time: time - delta - 1,
                });
            }
        }
    }
    best
}

pub fn part_a(input: &str) -> usize {
    // https://github.com/ephemient/aoc2022/blob/main/rs/src/day19.rs
    input
        .trim()
        .lines()
        .filter_map(|line| parse(line))
        .map(|(id, blueprint)| id * geodes(&blueprint, 24))
        .sum()
}

pub fn part_b(input: &str) -> usize {
    input
        .trim()
        .lines()
        .take(3)
        .filter_map(|line| parse(line))
        .map(|(_, blueprint)| geodes(&blueprint, 32))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 33);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 2193);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 7200);
    }
}
