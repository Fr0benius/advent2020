use regex::Regex;

use crate::parsing::*;

pub fn run() -> (usize, usize) {
    let input = include_str!("../input/02.txt");
    let reg = Regex::new(r#"(.*)-(.*) (.): (.*)"#).unwrap();
    let policies: Vec<(usize, usize, char, &str)> =
        input.lines().map(|line| parse_re(&reg, line)).collect();
    let part1 = policies.iter().filter(|&&(l, r, c, s)| {
        let k = s.chars().filter(|&d| d == c).count() as _;
        l <= k && k <= r
    }).count();
    println!("Part 1: {}", part1);
    let part2 = policies.iter().filter(|&&(l, r, c, s)| {
        let b = s.as_bytes();
        (b[l-1] == c as _) as i32 + (b[r-1] == c as _) as i32 == 1
    }).count();
    println!("Part 2: {}", part2);
    (part1, part2)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (467, 441));
    }
}

