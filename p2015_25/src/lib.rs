fn gen(input: i64) -> i64 {
    input * 252533 % 33554393
}

pub fn part_a() -> i64 {
    let mut num = 20151125;
    let row = 2981;
    let col = 3075;
    for i in 2.. {
        for j in 1..=i - 1 {
            if i - j == row && j == col {
                return num;
            }
            if !(i == 1 && j == 1) {
                num = gen(num);
            }
        }
    }
    panic!("No answer found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_a_result() {
        assert_eq!(part_a(), 9132360);
    }
}
