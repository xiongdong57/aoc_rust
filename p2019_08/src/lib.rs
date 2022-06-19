pub fn part_a(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let layers: Vec<&[char]> = chars.chunks(25 * 6).collect();
    let mut min_zeros = std::usize::MAX;
    let mut multiplied = 0;
    for layer in layers {
        let zeros = layer.iter().filter(|&&c| c == '0').count();
        if zeros < min_zeros {
            min_zeros = zeros;
            multiplied = layer.iter().filter(|&&c| c == '1').count()
                * layer.iter().filter(|&&c| c == '2').count();
        }
    }
    multiplied
}

fn calc_dots(layers: &[&[char]], i: i32, j: i32) -> char {
    let loc = j * 25 + i;
    for layer in layers {
        let c = layer[loc as usize];
        if c == '0' {
            return ' ';
        } else if c == '1' {
            return '#';
        }
    }
    ' '
}

pub fn part_b(input: &str) {
    let chars: Vec<char> = input.chars().collect();
    let layers: Vec<&[char]> = chars.chunks(25 * 6).collect();
    for j in 0..6 {
        for i in 0..25 {
            let dot = calc_dots(&layers, i, j);
            print!("{}", dot);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 2440);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        part_b(input);
    }
}
