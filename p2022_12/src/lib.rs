use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn char_to_num(c: &char) -> u8 {
    (c.to_owned() as u8) - ('a' as u8)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (i32, i32),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - // Fix it later
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
        // .then_with(|| other.position.cmp(&self.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// https://doc.rust-lang.org/stable/std/collections/binary_heap/index.html
fn shortest_path(heightmap: &HashMap<(i32, i32), u8>, start: (i32, i32), goal: (i32, i32)) -> Option<usize>{
    let dirs = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    // shortest_path
    let mut dist = HashMap::new();
    for k in heightmap.keys() {
        dist.insert(k, usize::MAX);
    }
    let mut heap = BinaryHeap::new();
    *dist.get_mut(&start).unwrap() = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }
        // Important as we may have already found a better way
        if cost > *dist.get(&position).unwrap() {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for dir in dirs {
            let (x, y) = position;
            let (dx, dy) = dir;
            let next_pos = (x + dx, y + dy);
            let next = State {
                cost: cost + 1,
                position: next_pos,
            };

            if (!heightmap.contains_key(&next_pos))
                || (heightmap.get(&next_pos).unwrap() > heightmap.get(&position).unwrap()
                    && ((heightmap.get(&next_pos).unwrap() - heightmap.get(&position).unwrap())
                        > 1))
            {
                continue;
            }
            // If so, add it to the frontier and continue
            if &next.cost < dist.get(&next_pos).unwrap() {
                heap.push(next);
                // Relaxation, we have now found a better way
                *dist.get_mut(&next_pos).unwrap() = next.cost;
            }
        }
    }
    None
}

pub fn part_a(input: &str) -> usize {
    let mut heightmap: HashMap<(i32, i32), u8> = HashMap::new();
    let mut start = (0, 0);
    let mut goal = (0, 0);

    for (j, line) in input.trim().lines().enumerate() {
        for (i, ch) in line.chars().enumerate() {
            let i = i as i32;
            let j = j as i32;
            if ch == 'S' {
                start = (i, j);
                heightmap.insert((i, j), char_to_num(&'a'));
            } else if ch == 'E' {
                goal = (i, j);
                heightmap.insert((i, j), char_to_num(&'z'));
            } else {
                heightmap.insert((i, j), char_to_num(&ch));
            }
        }
    }

    match shortest_path(&heightmap, start, goal) {
        Some(cost) => cost,
        None => panic!("no solution")
    }
}

pub fn part_b(input: &str) -> usize {
    let mut heightmap: HashMap<(i32, i32), u8> = HashMap::new();
    let mut starts = Vec::new();
    let mut goal = (0, 0);

    for (j, line) in input.trim().lines().enumerate() {
        for (i, ch) in line.chars().enumerate() {
            let i = i as i32;
            let j = j as i32;
            if ch == 'S' {
                starts.push((i, j));
                heightmap.insert((i, j), char_to_num(&'a'));
            } else if ch == 'E' {
                goal = (i, j);
                heightmap.insert((i, j), char_to_num(&'z'));
            } else {
                heightmap.insert((i, j), char_to_num(&ch));
                if ch == 'a' {
                    starts.push((i, j));
                }
            }
        }
    }

    let mut fewest_steps: usize = usize::MAX;
    for start in starts {
        if let Some(cost) = shortest_path(&heightmap, start, goal) {
            fewest_steps = fewest_steps.min(cost);
        }
    }
    fewest_steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 31);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 504);
    }

    #[test]
    fn part_b_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_b(input), 29);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 500);
    }
}
