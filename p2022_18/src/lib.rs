use std::{
    collections::{HashSet, VecDeque},
    ops::RangeInclusive,
};

#[derive(PartialEq, Eq, Hash)]
struct Cube {
    position: (i64, i64, i64),
    sides: [(i64, i64, i64); 6],
}

impl Cube {
    fn new(x: i64, y: i64, z: i64) -> Cube {
        let x1 = x * 2;
        let y1 = y * 2;
        let z1 = z * 2;
        Cube {
            position: (x, y, z),
            sides: [
                (x1 - 1, y1, z1),
                (x1 + 1, y1, z1),
                (x1, y1 - 1, z1),
                (x1, y1 + 1, z1),
                (x1, y1, z1 - 1),
                (x1, y1, z1 + 1),
            ],
        }
    }
    fn sides((x, y, z): (i64, i64, i64)) -> [(i64, i64, i64); 6] {
        [
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ]
    }
}
pub fn part_a(input: &str) -> usize {
    let cubes: Vec<Cube> = input
        .trim()
        .lines()
        .map(|line| {
            let ps: Vec<i64> = line.split(",").map(|p| p.parse::<i64>().unwrap()).collect();
            Cube::new(ps[0], ps[1], ps[2])
        })
        .collect();
    let sides = cubes
        .into_iter()
        .flat_map(|c| c.sides.into_iter().collect::<Vec<_>>())
        .collect::<Vec<(i64, i64, i64)>>();
    let n = sides.len();
    n - 2
        * (n - sides
            .iter()
            .copied()
            .collect::<HashSet<(i64, i64, i64)>>()
            .len())
}

fn fill(
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
    z_range: RangeInclusive<i64>,
    cubes: &HashSet<Cube>,
) -> HashSet<(i64, i64, i64)> {
    let start = (*x_range.start(), *y_range.start(), *z_range.start());
    let cube_positions = cubes.iter().map(|c| c.position).collect::<HashSet<_>>();
    let mut water = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);

    while let Some(position @ (x, y, z)) = queue.pop_front() {
        if x_range.contains(&x)
            && y_range.contains(&y)
            && z_range.contains(&z)
            && !water.contains(&position)
            && !cube_positions.contains(&position)
        {
            water.insert(position);
            queue.extend(Cube::sides(position));
        }
    }

    water
}

pub fn part_b(input: &str) -> i64 {
    // https://github.com/bsdrks/aoc2022/blob/main/src/bin/day18b.rs
    let cubes = input
        .trim()
        .lines()
        .map(|line| {
            let ps: Vec<i64> = line.split(",").map(|p| p.parse::<i64>().unwrap()).collect();
            Cube::new(ps[0], ps[1], ps[2])
        })
        .collect::<HashSet<Cube>>();
    let mut x_min = i64::MAX;
    let mut x_max = i64::MIN;
    let mut y_min = i64::MAX;
    let mut y_max = i64::MIN;
    let mut z_min = i64::MAX;
    let mut z_max = i64::MIN;

    for cube in &cubes {
        let (x, y, z) = cube.position;

        x_min = x_min.min(x);
        x_max = x_max.max(x);
        y_min = y_min.min(y);
        y_max = y_max.max(y);
        z_min = z_min.min(z);
        z_max = z_max.max(z);
    }
    let wet_sides = fill(
        x_min - 1..=x_max + 1,
        y_min - 1..=y_max + 1,
        z_min - 1..=z_max + 1,
        &cubes,
    )
    .into_iter()
    .map(|(x, y, z)| Cube::new(x, y, z))
    .collect::<HashSet<_>>()
    .iter()
    .flat_map(|c| c.sides)
    .collect::<HashSet<_>>();

    let mut n = 0;
    let sides = cubes.iter().flat_map(|c| c.sides).collect::<Vec<_>>();
    let mut shared = HashSet::new();

    for cube in &cubes {
        if cube.sides.iter().any(|s| wet_sides.contains(s)) {
            for side in &cube.sides {
                if wet_sides.contains(side) && !shared.contains(side) {
                    match sides.iter().filter(|s| s == &side).count() {
                        1 => n += 1,
                        2 => {
                            n += 1;

                            shared.insert(*side);
                        }
                        _ => (),
                    }
                }
            }
        }
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 64);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 3466);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 58);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 2012);
    }
}
