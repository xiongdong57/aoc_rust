use std::collections::{HashMap, HashSet};

use regex::Regex;

fn parse(input: &str) -> HashMap<(i32, i32), (i32, i32)> {
    let mut map = HashMap::new();
    let re =
        Regex::new(r"Sensor at x=(-*\d+), y=(-*\d+): closest beacon is at x=(-*\d+), y=(-*\d+)")
            .unwrap();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let a = caps
            .get(1)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        let b = caps
            .get(2)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        let x = caps
            .get(3)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        let y = caps
            .get(4)
            .map_or("", |m| m.as_str())
            .parse::<i32>()
            .unwrap();
        map.insert((a, b), (x, y));
    }
    map
}

pub fn part_a(input: &str, row: i32) -> usize {
    let map = parse(input);
    let mut occupy = HashSet::new();
    let becons = map
        .values()
        .map(|(x, y)| (*x, *y))
        .collect::<HashSet<(i32, i32)>>();
    for (k, v) in map.iter() {
        let (x, y) = k;
        let (dx, dy) = v;
        let distance = (x - dx).abs() + (y - dy).abs();

        if distance >= (row - y).abs() {
            let limit = distance - (row - y).abs();
            for i in (x - limit)..=(x + limit) {
                if !becons.contains(&(i, row)) {
                    occupy.insert(i);
                }
            }
        }
    }
    occupy.len()
}

pub fn part_b(input: &str, bound: i32) -> i64 {
    // https://www.reddit.com/r/adventofcode/comments/zmcn64/2022_day_15_solutions/
    // Part 2 in python in 0.01 seconds. Unprocessed input data in input_data.

    // import re
    // def all_numbers(s): return [int(d) for d in re.findall("(-?\d+)", s)]
    // def md(p, q): return abs(p[0]-q[0])+abs(p[1]-q[1])

    // data = [all_numbers(line) for line in input_data.split("\n")]
    // radius = {(a,b):md((a,b),(c,d)) for (a,b,c,d) in data}
    // scanners = radius.keys()

    // acoeffs, bcoeffs = set(), set()
    // for ((x,y), r) in radius.items():
    //     acoeffs.add(y-x+r+1)
    //     acoeffs.add(y-x-r-1)
    //     bcoeffs.add(x+y+r+1)
    //     bcoeffs.add(x+y-r-1)

    // bound = 4_000_000
    // for a in acoeffs:
    //     for b in bcoeffs:
    //         p = ((b-a)//2, (a+b)//2)
    //         if all(0<c<bound for c in p):
    //             if all(md(p,t)>radius[t] for t in scanners):
    //                 print(4_000_000*p[0]+p[1])

    // Here's the idea:

    // As there is only one missing value, it's going to be just outside the boundaries of at least two scanners
    // (unless we're incredibly unlucky and it's right on the bounds of the 0-4_000_000 square, but it isn't!).

    // The boundary of a scanner is four line segments. If a scanner is in position (sx,sy) and has 'radius' r,
    // then we want the line segments just outside, i.e. of radius r+1. There will be two line segments of gradient 1:

    // y = x + sy-sx+r+1
    // y = x + sy-sx-r-1

    // and two line segments of gradient -1:

    // y = -x + sx+sy+r+1
    // y = -x + sx+sy-r-1

    // Determining where a line y=x+a and a line y=-x+b intersect is very easy - they intersect at the point ( (b-a)/2 , (a+b)/2 ).

    // One of these intersection points will be the missing scanner location.
    //  So, we assemble a set of all the 'a' coefficients (lines of gradient 1) and all the 'b' coefficients (lines of gradient -1),
    //  then look at their intersections to see if they are the point we need.
    // Given the number of scanners we only need to check a couple of thousand points at most.
    let map = parse(input);
    let mut acoeffs = HashSet::new();
    let mut bcoeffs = HashSet::new();
    let mut radius = HashMap::new();
    for (k, v) in map.iter() {
        let (sx, sy) = k;
        let (dx, dy) = v;
        let r = (sx - dx).abs() + (sy - dy).abs();
        acoeffs.insert(sy - sx + r + 1);
        acoeffs.insert(sy - sx - r - 1);
        bcoeffs.insert(sx + sy + r + 1);
        bcoeffs.insert(sx + sy - r - 1);
        radius.insert(k, r);
    }
    for a in acoeffs.iter() {
        for b in bcoeffs.iter() {
            let x = (b - a) / 2;
            let y = (b + a) / 2;
            if x >= 0 && x <= bound && y >= 0 && y <= bound {
                if radius
                    .iter()
                    .all(|((sx, sy), r)| ((sx - x).abs() + (sy - y).abs()) > *r)
                {
                    return 4_000_000 * (x as i64) + (y as i64);
                }
            }
        }
    }
    panic!("no solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input, 10), 26);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input, 2000000), 4560025);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input, 20), 56000011);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input, 4000000), 12480406634249);
    }
}
