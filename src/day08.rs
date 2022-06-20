use std::collections::HashSet;

use crate::parsing::Extract;

pub fn run() -> (i64, i64) {
    let input = include_str!("../input/08.txt");
    let instrs: Vec<(&str, i64)> = input
        .lines()
        .map(|line| Extract::extract(&mut line.split_whitespace()))
        .collect();
    let part1 = simulate(&instrs, None).unwrap_err();
    println!("Part 1: {part1}");
    let part2 = (0..instrs.len())
        .filter(|&i| instrs[i].0 != "acc")
        .map(|i| simulate(&instrs, Some(i)))
        .find_map(|res| res.ok())
        .unwrap();
    println!("Part 2: {part2}");
    (part1, part2)
}

fn simulate(instrs: &[(&str, i64)], flip: Option<usize>) -> Result<i64, i64> {
    let mut seen = HashSet::new();
    let mut ix = 0;
    let mut acc = 0;
    loop {
        if !seen.insert(ix) {
            return Err(acc);
        }
        if !(0..instrs.len()).contains(&ix) {
            return Ok(acc);
        }
        let (mut op, k) = instrs[ix];
        if flip == Some(ix) {
            op = if op == "nop" { "jmp" } else { "nop" };
        }
        match op {
            "acc" => acc += k,
            "jmp" => ix = (ix as i64 + k - 1) as _,
            _ => (),
        }
        ix += 1;
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (1217, 501));
    }
}
