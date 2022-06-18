use serde_json::{Map, Value};

pub fn add_num(val: Value) -> i64 {
    let mut sum = 0;
    match val {
        Value::Number(num) => sum += num.as_i64().unwrap(),
        Value::Array(vals) => {
            for v in vals {
                sum += add_num(v);
            }
        }
        Value::Object(obj) => {
            for (_, v) in obj {
                sum += add_num(v);
            }
        }
        Value::Bool(_) | Value::String(_) | Value::Null => {}
    }

    sum
}

pub fn part_a(input: &str) -> i64 {
    let json = serde_json::from_str::<Map<String, Value>>(input).unwrap();
    let mut sum = 0;
    for (_, v) in json {
        sum += add_num(v);
    }
    sum
}

pub fn add_num_v2(val: Value) -> i64 {
    let mut sum = 0;
    match val {
        Value::Number(num) => sum += num.as_i64().unwrap(),
        Value::Array(vals) => {
            for v in vals {
                sum += add_num_v2(v);
            }
        }
        Value::Object(obj) => {
            for (_, v) in obj {
                if let Value::String(s) = &v {
                    if s == "red" {
                        return 0;
                    }
                }
                sum += add_num_v2(v);
            }
        }
        Value::Bool(_) | Value::String(_) | Value::Null => {}
    }

    sum
}

pub fn part_b(input: &str) -> i64 {
    let json = serde_json::from_str::<Map<String, Value>>(input).unwrap();
    let mut sum = 0;
    for (_, v) in json {
        sum += add_num_v2(v);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 111754);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 65402);
    }
}
