use std::collections::HashMap;

// use std::str::FromStr;
#[derive(Debug)]
enum Instruction {
    HLF(char),
    TPL(char),
    INC(char),
    JMP(i64),
    JIE(char, i64),
    JIO(char, i64),
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.to_owned();
        let parts = &s.split_once(" ").unwrap();
        match parts.0 {
            "hlf" => Ok(Instruction::HLF(parts.1.parse().unwrap())),
            "tpl" => Ok(Instruction::TPL(parts.1.parse().unwrap())),
            "inc" => Ok(Instruction::INC(parts.1.parse().unwrap())),
            "jmp" => Ok(Instruction::JMP(parts.1.parse::<i64>().unwrap())),
            "jie" => {
                let (reg, offset) = parts.1.split_once(", ").unwrap();
                Ok(Instruction::JIE(
                    reg.parse().unwrap(),
                    offset.parse().unwrap(),
                ))
            }
            "jio" => {
                let (reg, offset) = parts.1.split_once(", ").unwrap();
                Ok(Instruction::JIO(
                    reg.parse().unwrap(),
                    offset.parse().unwrap(),
                ))
            }
            _ => Err("Invalid instruction"),
        }
    }
}

pub fn part_a(input: &str) -> i64 {
    let instructions = input
        .lines()
        .map(|l| Instruction::try_from(l).unwrap())
        .collect::<Vec<_>>();

    let mut reg = HashMap::new();
    reg.insert('a', 0);
    reg.insert('b', 0);
    let mut curr = 0;
    while let Some(cmd) = instructions.get(curr as usize) {
        match cmd {
            Instruction::HLF(c) => {
                *reg.get_mut(c).unwrap() /= 2;
            }
            Instruction::TPL(c) => {
                *reg.get_mut(c).unwrap() *= 3;
            }
            Instruction::INC(c) => {
                *reg.get_mut(c).unwrap() += 1;
            }
            Instruction::JMP(offset) => {
                curr += offset - 1;
            }
            Instruction::JIE(c, offset) => {
                if *reg.get(c).unwrap() % 2 == 0 {
                    curr += offset - 1;
                }
            }
            Instruction::JIO(c, offset) => {
                if *reg.get(c).unwrap() == 1 {
                    curr += offset - 1;
                }
            }
        }
        curr += 1;
    }
    reg[&'b']
}

pub fn part_b(input: &str) -> i64 {
    let instructions = input
        .lines()
        .map(|l| Instruction::try_from(l).unwrap())
        .collect::<Vec<_>>();

    let mut reg = HashMap::new();
    reg.insert('a', 1);
    reg.insert('b', 0);
    let mut curr = 0;
    while let Some(cmd) = instructions.get(curr as usize) {
        match cmd {
            Instruction::HLF(c) => {
                *reg.get_mut(c).unwrap() /= 2;
            }
            Instruction::TPL(c) => {
                *reg.get_mut(c).unwrap() *= 3;
            }
            Instruction::INC(c) => {
                *reg.get_mut(c).unwrap() += 1;
            }
            Instruction::JMP(offset) => {
                curr += offset - 1;
            }
            Instruction::JIE(c, offset) => {
                if *reg.get(c).unwrap() % 2 == 0 {
                    curr += offset - 1;
                }
            }
            Instruction::JIO(c, offset) => {
                if *reg.get(c).unwrap() == 1 {
                    curr += offset - 1;
                }
            }
        }
        curr += 1;
    }
    reg[&'b']
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_a(input), 170);
    }

    #[test]
    fn part_b_result() {
        let input = include_str!("input.txt");
        assert_eq!(part_b(input), 247);
    }
}
