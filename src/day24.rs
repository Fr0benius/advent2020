use std::collections::HashMap;

use Dir::{E, NE, NW, SE, SW, W};


pub fn run() -> (usize, usize) {
    let input = include_str!("../input/24.txt");
    let paths: Vec<Vec<Dir>> = input.lines()
        .map(parse_dirs).collect();
    let part1 = {
        let mut flips = HashMap::new();
        for path in &paths {
            let cell = path.iter().copied().fold((0, 0), step);
            *flips.entry(cell).or_insert(0usize) += 1;
        }
        flips.into_values().filter(|&n| n % 2 == 1).count()
    };
    let part2 = 0;
    (part1, part2)
}

type Pt = (i64, i64);

#[derive(Clone, Copy, Debug)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

fn step((x, y): Pt, dir: Dir) -> Pt {
    match dir {
        E => (x + 1, y),
        SE => (x + 1, y - 1),
        SW => (x, y - 1),
        W => (x - 1, y),
        NW => (x - 1, y + 1),
        NE => (x, y + 1),
    }
}

fn parse_dirs(s: &str) -> Vec<Dir> {
    let mut it = s.chars();
    let mut res = vec![];
    while let Some(c) = it.next() {
        let dir = match c {
            'e' => E,
            'w' => W,
            _ => {
                match (c, it.next().unwrap()) {
                    ('n', 'w') => NW,
                    ('n', 'e') => NE,
                    ('s', 'w') => SW,
                    ('s', 'e') => SE,
                    _ => unreachable!(),
                }
            }
        };
        res.push(dir);
    }
    res
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (228, 186_715_244_496));
    }
}
