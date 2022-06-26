use std::collections::{HashMap, HashSet};

use Dir::{E, NE, NW, SE, SW, W};

pub fn run() -> (usize, usize) {
    let input = include_str!("../input/24.txt");
    let paths: Vec<Vec<Dir>> = input.lines().map(parse_dirs).collect();
    let starting_cells: HashSet<Pt> = {
        let mut flips = HashMap::new();
        for path in &paths {
            let cell = path.iter().copied().fold((0, 0), step);
            *flips.entry(cell).or_insert(0usize) += 1;
        }
        flips
            .into_iter()
            .filter(|&(_, n)| n % 2 == 1)
            .map(|(a, _)| a)
            .collect()
    };

    let mut cells = starting_cells.clone();
    for _ in 0..100 {
        let candidates: HashSet<_> = cells.iter().copied().flat_map(neighbors).collect();
        cells = candidates
            .into_iter()
            .filter(|&pt| {
                let alive = cells.contains(&pt);
                let k = neighbors(pt).filter(|q| cells.contains(q)).count();
                alive && [1, 2].contains(&k) || !alive && k == 2
            })
            .collect();
    }
    (starting_cells.len(), cells.len())
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

fn neighbors(pt: Pt) -> impl Iterator<Item = Pt> {
    [E, SE, SW, W, NW, NE]
        .map(move |dir| step(pt, dir))
        .into_iter()
}

fn parse_dirs(s: &str) -> Vec<Dir> {
    let mut it = s.chars();
    let mut res = vec![];
    while let Some(c) = it.next() {
        let dir = match c {
            'e' => E,
            'w' => W,
            _ => match (c, it.next().unwrap()) {
                ('n', 'w') => NW,
                ('n', 'e') => NE,
                ('s', 'w') => SW,
                ('s', 'e') => SE,
                _ => unreachable!(),
            },
        };
        res.push(dir);
    }
    res
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (228, 3672));
    }
}
