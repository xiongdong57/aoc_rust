use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<&str, HashMap<&str, i64>> {
    let mut ingredient = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let name = parts[0];
        let mut properties = HashMap::new();
        for prop in parts[1].split(", ") {
            let mut prop_parts = prop.split(" ");
            let prop_name = prop_parts.next().unwrap();
            let prop_value = prop_parts.next().unwrap().parse::<i64>().unwrap();
            properties.insert(prop_name, prop_value);
        }
        ingredient.insert(name, properties);
    }
    ingredient
}

fn calc_score(
    ingredient: &HashMap<&str, HashMap<&str, i64>>,
    recipe: &HashMap<&str, i64>,
    elements: &Vec<&str>,
) -> i64 {
    let mut score = 1;
    for prop in &["capacity", "durability", "flavor", "texture"] {
        let mut prop_score = 0;
        for elem in elements {
            let count = recipe.get(elem).unwrap();
            prop_score += count * ingredient.get(elem).unwrap().get(prop).unwrap();
        }
        score = score * (0.max(prop_score));
    }
    score
}

pub fn part_a(input: &str) -> i64 {
    let ingredient = parse_input(input);
    let elements: Vec<&str> = ingredient.keys().copied().collect();
    let mut max_score = 0;
    for i in 0..=100 {
        for j in 0..=100 - i {
            for k in 0..=100 - i - j {
                let l = 100 - i - j - k;

                let mut recipe = HashMap::new();
                let counts = vec![i as i64, j, k, l];
                for (k, v) in elements.iter().zip(counts.iter()) {
                    recipe.insert(*k, *v);
                }

                let score = calc_score(&ingredient, &recipe, &elements);
                max_score = max_score.max(score);
            }
        }
    }

    max_score
}

pub fn part_b(input: &str) -> i64 {
    let ingredient = parse_input(input);
    let elements: Vec<&str> = ingredient.keys().copied().collect();
    let mut max_score = 0;
    for i in 0..=100 {
        for j in 0..=100 - i {
            for k in 0..=100 - i - j {
                let l = 100 - i - j - k;

                let mut recipe = HashMap::new();
                let counts = vec![i as i64, j, k, l];
                let mut total_calories = 0;
                for (k, v) in elements.iter().zip(counts.iter()) {
                    recipe.insert(*k, *v);
                    total_calories += v * ingredient.get(k).unwrap().get("calories").unwrap();
                }

                if total_calories == 500 {
                    let score = calc_score(&ingredient, &recipe, &elements);
                    max_score = max_score.max(score);
                }
            }
        }
    }

    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 18965440);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 15862900);
    }
}
