use std::collections::HashMap;

use crate::parsing::parse_iter;

pub fn run() -> (usize, usize) {
    let input = include_str!("../input/15.txt").trim();
    let mut v: Vec<usize> = parse_iter(&mut input.split(','));
    let mut pos: HashMap<_, _> = 
        v.iter()
        .enumerate()
        .map(|(i, &k)| (k, i))
        .collect();
    let (ix1, ix2) = (2020, 30_000_000);
    for n in v.len()..ix2 {
        let &k = v.last().unwrap();
        let j = pos.insert(k, n - 1).unwrap_or(n - 1);
        v.push(n - 1 - j);
    };
    dbg!((v[ix1 - 1], v[ix2 - 1]))
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (441, 10_613_991));
    }
}
