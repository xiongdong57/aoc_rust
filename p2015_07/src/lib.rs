use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
enum Instruction {
    SetNum(u16, String),
    SetReg(String, String),
    AND(String, String, String),
    NumAdd(u16, String, String),
    OR(String, String, String),
    LSHIFT(String, u16, String),
    RSHIFT(String, u16, String),
    NOT(String, String),
}

impl Instruction {
    fn input_nodes(&self) -> Vec<String> {
        match self {
            Instruction::SetNum(_, _) => vec![],
            Instruction::SetReg(x, _) => vec![x.clone()],
            Instruction::AND(s1, s2, _) => vec![s1.clone(), s2.clone()],
            Instruction::NumAdd(_, s1, _) => vec![s1.clone()],
            Instruction::OR(s1, s2, _) => vec![s1.clone(), s2.clone()],
            Instruction::LSHIFT(s1, _, _) => vec![s1.clone()],
            Instruction::RSHIFT(s1, _, _) => vec![s1.clone()],
            Instruction::NOT(s1, _) => vec![s1.clone()],
        }
    }

    fn output_nodes(&self) -> String {
        match self {
            Instruction::SetNum(_, s) => s.clone(),
            Instruction::SetReg(_, s) => s.clone(),
            Instruction::AND(_, _, s) => s.clone(),
            Instruction::NumAdd(_, _, s) => s.clone(),
            Instruction::OR(_, _, s) => s.clone(),
            Instruction::LSHIFT(_, _, s) => s.clone(),
            Instruction::RSHIFT(_, _, s) => s.clone(),
            Instruction::NOT(_, s) => s.clone(),
        }
    }
    fn exe(&self, comp: &mut HashMap<String, u16>) {
        match self {
            Instruction::SetNum(num, out) => {
                comp.insert(out.clone(), *num);
            }
            Instruction::SetReg(inp, out) => {
                comp.insert(out.clone(), comp[inp]);
            }
            Instruction::AND(in1, in2, out) => {
                comp.insert(out.clone(), comp[in1] & comp[in2]);
            }
            Instruction::NumAdd(num, inp, out) => {
                comp.insert(out.clone(), comp[inp] & num);
            }
            Instruction::OR(in1, in2, out) => {
                comp.insert(out.clone(), comp[in1] | comp[in2]);
            }
            Instruction::LSHIFT(in1, num, out) => {
                comp.insert(out.clone(), comp[in1] << *num);
            }
            Instruction::RSHIFT(in1, num, out) => {
                comp.insert(out.clone(), comp[in1] >> *num);
            }
            Instruction::NOT(in1, out) => {
                comp.insert(out.clone(), !comp[in1]);
            }
        }
    }
}

pub fn part_a(input: &str) -> u16 {
    let mut remaining = VecDeque::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split(' ').collect();
        let inst = match (parts[0], parts[1]) {
            (_, "->") => {
                if let Ok(num) = parts[0].parse::<u16>() {
                    Instruction::SetNum(num, parts[2].to_string())
                } else {
                    Instruction::SetReg(parts[0].to_string(), parts[2].to_string())
                }
            }
            (_, "AND") => {
                if let Ok(num) = parts[0].parse::<u16>() {
                    Instruction::NumAdd(num, parts[2].to_string(), parts[4].to_string())
                } else {
                    Instruction::AND(
                        parts[0].to_string(),
                        parts[2].to_string(),
                        parts[4].to_string(),
                    )
                }
            }
            (_, "OR") => Instruction::OR(
                parts[0].to_string(),
                parts[2].to_string(),
                parts[4].to_string(),
            ),
            (_, "LSHIFT") => Instruction::LSHIFT(
                parts[0].to_string(),
                parts[2].parse::<u16>().unwrap(),
                parts[4].to_string(),
            ),
            (_, "RSHIFT") => Instruction::RSHIFT(
                parts[0].to_string(),
                parts[2].parse::<u16>().unwrap(),
                parts[4].to_string(),
            ),
            ("NOT", _) => Instruction::NOT(parts[1].to_string(), parts[3].to_string()),
            _ => panic!("Unknown instruction: {}", line),
        };
        remaining.push_front(inst);
    }

    let mut sorted = Vec::new();
    let mut seen_nodes = HashSet::new();
    while let Some(inst) = remaining.pop_front() {
        // if input of next is empty or seen then add next to sorted,
        // otherwise add it back to remaining
        if inst.input_nodes().is_empty() {
            seen_nodes.insert(inst.output_nodes());
            sorted.push(inst);
        } else if inst
            .input_nodes()
            .into_iter()
            .all(|s| seen_nodes.contains(&s))
        {
            seen_nodes.insert(inst.output_nodes());
            sorted.push(inst);
        } else {
            remaining.push_back(inst);
        }
    }

    // execute instructions in order
    let mut comp = HashMap::new();
    for inst in sorted {
        inst.exe(&mut comp);
    }

    comp["a"]
}

pub fn part_b(input: &str) -> u16 {
    let mut remaining = VecDeque::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split(' ').collect();
        let inst = match (parts[0], parts[1]) {
            (_, "->") => {
                if let Ok(num) = parts[0].parse::<u16>() {
                    Instruction::SetNum(num, parts[2].to_string())
                } else {
                    Instruction::SetReg(parts[0].to_string(), parts[2].to_string())
                }
            }
            (_, "AND") => {
                if let Ok(num) = parts[0].parse::<u16>() {
                    Instruction::NumAdd(num, parts[2].to_string(), parts[4].to_string())
                } else {
                    Instruction::AND(
                        parts[0].to_string(),
                        parts[2].to_string(),
                        parts[4].to_string(),
                    )
                }
            }
            (_, "OR") => Instruction::OR(
                parts[0].to_string(),
                parts[2].to_string(),
                parts[4].to_string(),
            ),
            (_, "LSHIFT") => Instruction::LSHIFT(
                parts[0].to_string(),
                parts[2].parse::<u16>().unwrap(),
                parts[4].to_string(),
            ),
            (_, "RSHIFT") => Instruction::RSHIFT(
                parts[0].to_string(),
                parts[2].parse::<u16>().unwrap(),
                parts[4].to_string(),
            ),
            ("NOT", _) => Instruction::NOT(parts[1].to_string(), parts[3].to_string()),
            _ => panic!("Unknown instruction: {}", line),
        };
        remaining.push_front(inst);
    }

    let mut sorted = Vec::new();
    let mut seen_nodes = HashSet::new();
    while let Some(inst) = remaining.pop_front() {
        // if input of next is empty or seen then add next to sorted,
        // otherwise add it back to remaining
        if inst.input_nodes().is_empty() {
            seen_nodes.insert(inst.output_nodes());
            sorted.push(inst);
        } else if inst
            .input_nodes()
            .into_iter()
            .all(|s| seen_nodes.contains(&s))
        {
            seen_nodes.insert(inst.output_nodes());
            sorted.push(inst);
        } else {
            remaining.push_back(inst);
        }
    }

    // execute instructions in order
    let mut comp = HashMap::new();
    for inst in &sorted {
        inst.exe(&mut comp);
    }

    let mut comp2 = HashMap::new();
    comp2.insert("b".to_string(), comp["a"]);
    for inst in &sorted {
        if let Instruction::SetNum(_, output) = inst {
            if output == "b" {
                continue;
            }
        }
        inst.exe(&mut comp2);
    }

    comp2["a"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 46065);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 14134);
    }
}
