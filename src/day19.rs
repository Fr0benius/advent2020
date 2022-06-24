use std::collections::HashMap;

use crate::parsing::Gather;

pub fn run() -> (usize, usize) {
    let input = include_str!("../input/19.txt");
    let mut memo: HashMap<(usize, &str), bool> = HashMap::new();
    let (mut rules, msgs) = {
        let (blob1, blob2): (&str, &str) = input.split("\n\n").gather();
        let rules: HashMap<usize, Vec<Vec<usize>>> = blob1
            .lines()
            .map(|l| {
                let (k, s): (usize, &str) = l.split(": ").gather();
                if s.starts_with('"') {
                    memo.insert(dbg!((k, &s[1..2])), true);
                    (k, vec![])
                } else {
                    let v = s
                        .split(" | ")
                        .map(|t| t.split_whitespace().gather())
                        .collect();
                    (k, v)
                }
            })
            .collect();
        let msgs: Vec<&str> = blob2.lines().collect();
        (rules, msgs)
    };
    let memo_orig = memo.clone();
    let part1 = dbg!(msgs
        .iter()
        .filter(|&&msg| fits(0, msg, &rules, &mut memo))
        .count());
    dbg!(memo.len());
    rules.insert(8, vec![vec![42], vec![42, 8]]);
    rules.insert(11, vec![vec![42, 31], vec![42, 11, 31]]);
    memo = memo_orig;
    let part2 = dbg!(msgs
        .iter()
        .filter(|&&msg| fits(0, msg, &rules, &mut memo))
        .count());
    dbg!(memo.len());
    (part1, part2)
}

fn fits<'a>(
    k: usize,
    s: &'a str,
    rules: &HashMap<usize, Vec<Vec<usize>>>,
    memo: &mut HashMap<(usize, &'a str), bool>,
) -> bool {
    assert!(!s.is_empty());
    if let Some(&res) = memo.get(&(k, s)) {
        return res;
    }
    for r in &rules[&k] {
        // TODO: Handle splits without special cases
        if r.len() == 1 {
            if fits(r[0], s, rules, memo) {
                memo.insert((k, s), true);
                return true;
            }
        } else if r.len() == 2 {
            for i in 1..s.len() {
                let (s1, s2) = s.split_at(i);
                if fits(r[0], s1, rules, memo) && fits(r[1], s2, rules, memo) {
                    memo.insert((k, s), true);
                    return true;
                }
            }
        } else if r.len() == 3 {
            for i in 1..s.len() {
                for j in i + 1..s.len() {
                    let (s2, s3) = s.split_at(j);
                    let (s1, s2) = s2.split_at(i);
                    if fits(r[0], s1, rules, memo)
                        && fits(r[1], s2, rules, memo)
                        && fits(r[2], s3, rules, memo)
                    {
                        memo.insert((k, s), true);
                        return true;
                    }
                }
            }
        } else {
            panic!("Rule has more than 3 splits");
        }
    }
    memo.insert((k, s), false);
    false
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (299, 414));
    }
}
