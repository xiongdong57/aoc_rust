use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Open,
    Solid,
}

#[derive(Debug)]
enum Move {
    Left,
    Right,
    Foward(i32),
}

#[derive(Debug)]
struct Face {
    x: i32,
    y: i32,
}

impl Face {
    fn turn_right(&self) -> Face {
        Face {
            x: -self.y,
            y: self.x,
        }
    }

    fn turn_left(&self) -> Face {
        Face {
            x: self.y,
            y: -self.x,
        }
    }
    fn value(&self) -> i32 {
        let (x, y) = (self.x, self.y);
        if (x, y) == (1, 0) {
            0
        } else if (x, y) == (0, 1) {
            1
        } else if (x, y) == (-1, 0) {
            2
        } else if (x, y) == (0, -1) {
            3
        } else {
            panic!("wrong direction")
        }
    }
}

fn parse(input: &str) -> (Vec<Move>, HashMap<(i32, i32), Tile>) {
    let mut map = HashMap::new();
    let (pa, pb) = input.split_once("\r\n\r\n").unwrap();
    for (y, line) in pa.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '.' {
                map.insert((x as i32, y as i32), Tile::Open);
            } else if ch == '#' {
                map.insert((x as i32, y as i32), Tile::Solid);
            }
        }
    }
    let mut moves = Vec::new();
    let mut temp = Vec::new();
    for ch in pb.chars() {
        if ch == 'R' {
            if temp.len() > 0 {
                let num = temp.join("").parse::<i32>().unwrap();
                moves.push(Move::Foward(num));
                temp.clear();
            }
            moves.push(Move::Right);
        } else if ch == 'L' {
            if temp.len() > 0 {
                let num = temp.join("").parse::<i32>().unwrap();
                moves.push(Move::Foward(num));
                temp.clear();
            }
            moves.push(Move::Left);
        } else {
            temp.push(ch.to_string());
        }
    }
    if temp.len() > 0 {
        let num = temp.join("").parse::<i32>().unwrap();
        moves.push(Move::Foward(num));
        temp.clear();
    }
    (moves, map)
}

fn simulate_move(
    map: &HashMap<(i32, i32), Tile>,
    cur_idx: (i32, i32),
    steps: i32,
    face: &Face,
) -> (i32, i32) {
    let (mut x, mut y) = cur_idx;
    let (dx, dy) = (face.x, face.y);
    let x_min = *map
        .keys()
        .filter(|(_, my)| my == &y)
        .map(|(mx, _)| mx)
        .min()
        .unwrap();
    let x_max = *map
        .keys()
        .filter(|(_, my)| my == &y)
        .map(|(mx, _)| mx)
        .max()
        .unwrap();
    let y_min = *map
        .keys()
        .filter(|(mx, _)| mx == &x)
        .map(|(_, my)| my)
        .min()
        .unwrap();
    let y_max = *map
        .keys()
        .filter(|(mx, _)| mx == &x)
        .map(|(_, my)| my)
        .max()
        .unwrap();
    for _ in 0..steps {
        let mut next_x = x + dx;
        let mut next_y = y + dy;
        if next_x < x_min {
            next_x = x_max;
        }
        if next_x > x_max {
            next_x = x_min;
        }
        if next_y < y_min {
            next_y = y_max;
        }
        if next_y > y_max {
            next_y = y_min;
        }
        if *map.get(&(next_x, next_y)).unwrap() == Tile::Solid {
            return (x, y);
        } else {
            x = next_x;
            y = next_y;
        }
    }
    (x, y)
}

pub fn part_a(input: &str) -> i32 {
    let (moves, map) = parse(input);
    let mut face = Face { x: 1, y: 0 };
    let mut cur_idx = *map.keys().filter(|(_, y)| y == &0).min().unwrap();
    for step in moves {
        match step {
            Move::Left => face = face.turn_left(),
            Move::Right => face = face.turn_right(),
            Move::Foward(num) => cur_idx = simulate_move(&map, cur_idx, num, &face),
        }
    }
    1000 * (cur_idx.1 + 1) + 4 * (cur_idx.0 + 1) + face.value()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_works() {
        let input = include_str!("test.txt");
        assert_eq!(part_a(input), 6032);
    }

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 57350);
    }
}
