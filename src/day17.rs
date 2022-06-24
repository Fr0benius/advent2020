use std::collections::HashSet;

pub fn run() -> (usize, usize) {
    let input = include_str!("../input/17.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let n = grid.len();
    let m = grid[0].len();
    let mut init: HashSet<Pt3> = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '#' {
                init.insert((i as _, j as _, 0));
            }
        }
    }
    let mut state2 = init.iter().map(|&(a, b, c)| (a, b, c, 0)).collect();
    let mut state = init;
    for _ in 0..6 {
        state = step(&state);
        state2 = step(&state2);
    }
    (state.len(), state2.len())
}

fn step<T: Pt + Eq + std::hash::Hash>(state: &HashSet<T>) -> HashSet<T> {
    state
        .iter()
        .flat_map(Pt::neighbors)
        .filter(|pt| {
            let k = pt.neighbors().iter().filter(|q| state.contains(q)).count();
            if state.contains(pt) {
                k == 2 || k == 3
            } else {
                k == 3
            }
        })
        .collect()
}

trait Pt: Sized {
    fn neighbors(&self) -> Vec<Self>;
}

type Pt3 = (i64, i64, i64);
impl Pt for Pt3 {
    fn neighbors(&self) -> Vec<Self> {
        let &(x0, y0, z0) = self;
        let mut res = vec![];
        for x in x0 - 1..=x0 + 1 {
            for y in y0 - 1..=y0 + 1 {
                for z in z0 - 1..=z0 + 1 {
                    if (x, y, z) != (x0, y0, z0) {
                        res.push((x, y, z));
                    }
                }
            }
        }
        res
    }
}

type Pt4 = (i64, i64, i64, i64);
impl Pt for Pt4 {
    fn neighbors(&self) -> Vec<Self> {
        let &(x0, y0, z0, w0) = self;
        let mut res = vec![];
        for x in x0 - 1..=x0 + 1 {
            for y in y0 - 1..=y0 + 1 {
                for z in z0 - 1..=z0 + 1 {
                    for w in w0 - 1..=w0 + 1 {
                        if (x, y, z, w) != (x0, y0, z0, w0) {
                            res.push((x, y, z, w));
                        }
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (375, 2192));
    }
}
