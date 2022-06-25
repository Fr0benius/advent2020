use std::collections::HashSet;

use crate::parsing::Gather;

pub fn run() -> (usize, usize)  {
    let input = include_str!("../input/22.txt");
    let decks: Vec<Vec<usize>> = {
        input
            .split("\n\n")
            .map(|blob| {
                // dbg!(blob);
                blob.lines().skip(1).gather()
            })
            .collect()
    };
    let part1 = dbg!(simulate(&decks[0], &decks[1], false).1);
    let part2 = dbg!(simulate(&decks[0], &decks[1], true).1);
    (part1, part2)
}

fn simulate(v1: &[usize], v2: &[usize], recur: bool) -> (usize, usize) {
    let mut memo: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();
    let mut deck1 = v1.to_owned();
    let mut deck2 = v2.to_owned();
    while !(deck1.is_empty() || deck2.is_empty()) {
        if !memo.insert((deck1.clone(), deck2.clone())) {
            return (0, 0);
        }
        let c1 = deck1.remove(0);
        let c2 = deck2.remove(0);
        let winner = {
            if recur && c1 <= deck1.len() && c2 <= deck2.len() {
                simulate(&deck1[..c1], &deck2[..c2], true).0
            } else if c1 > c2 {
                0
            } else {
                1
            }
        };
        if winner == 0 {
            deck1.extend(&[c1, c2]);
        } else {
            deck2.extend(&[c2, c1]);
        }
    }
    if deck1.is_empty() {
        (1, score(&deck2))
    } else {
        (0, score(&deck1))
    }
}

fn score(d: &[usize]) -> usize {
    let n = d.len();
    d.iter()
        .enumerate()
        .map(|(i, &k)| k * (n - i) as usize)
        .sum()
}
#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (33772, 35070));
    }
}
