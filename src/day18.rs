use crate::shunting_yard::{*, Op::{Add, Mul}};

pub fn run() -> (i64, i64) {
    let input = include_str!("../input/18.txt");
    let exprs: Vec<Vec<Token>> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
                .map(Token::from)
                .collect()
        })
        .collect();
    let mut yard = ShuntingYard::new([(Add, 1), (Mul, 1)].into());
    let part1 = exprs.iter().map(|expr| yard.eval(expr)).sum();
    let mut yard = ShuntingYard::new([(Add, 2), (Mul, 1)].into());
    let part2 = exprs.iter().map(|expr| yard.eval(expr)).sum();
    (part1, part2)
}


#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (5_374_004_645_253, 88_782_789_402_798));
    }
}
