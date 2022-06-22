#[allow(clippy::needless_range_loop)]
pub fn run() -> (usize, usize) {
    let grid: Vec<Vec<char>> = include_str!("../input/11.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let n = grid.len();
    let m = grid[0].len();
    let mut neighbors: Vec<Vec<Vec<_>>> = vec![vec![vec![]; m]; n];
    for r in 0..n {
        for c in 0..m {
            neighbors[r][c] = neighbors8(r, c, n, m).collect();
        }
    }
    let part1 = dbg!(simulate(&grid, &neighbors, 4));
    for r in 0..n {
        for c in 0..m {
            neighbors[r][c].clear();
            for (dr, dc) in DIR8 {
                let mut rx = r as i64 + dr;
                let mut cx = c as i64 + dc;
                while (rx as usize) < n &&
                    (cx as usize) < m &&
                    grid[rx as usize][cx as usize] == '.'
                    {
                        rx += dr;
                        cx += dc;
                    }
                let (rx, cx) = (rx as usize, cx as usize);
                if rx < n && cx < m {
                    neighbors[r][c].push((rx, cx));
                }
            }
        }
    }
    let part2 = dbg!(simulate(&grid, &neighbors, 5));
    (part1, part2)
}

fn simulate(grid: &Vec<Vec<char>>, neighbors: &[Vec<Vec<(usize, usize)>>], k: usize) -> usize {
    let n = grid.len();
    let m = grid[0].len();
    let mut prv = grid.clone();
    loop {
        let nxt: Vec<Vec<char>> = (0..n)
            .map(|r| {
                (0..m)
                    .map(|c| {
                        if prv[r][c] == 'L' && neighbors[r][c].iter().all(|&(a, b)| prv[a][b] != '#')
                        {
                            '#'
                        } else if prv[r][c] == '#'
                            && neighbors[r][c].iter()
                                .filter(|&&(a, b)| prv[a][b] == '#')
                                .count()
                                >= k
                        {
                            'L'
                        } else {
                            prv[r][c]
                        }
                    })
                    .collect()
            })
            .collect();
        if nxt == prv {
            return prv.iter().flatten().filter(|&&c| c == '#').count();
        }
        prv = nxt;
    }
}

pub const DIR4: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
pub const DIR8: [(i64, i64); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

pub fn neighbors_iter(
    dirs: &'static [(i64, i64)],
    r: usize,
    c: usize,
    n: usize,
    m: usize,
) -> impl Iterator<Item = (usize, usize)> {
    dirs.iter()
        .map(move |(dr, dc)| ((r as i64 + dr) as usize, (c as i64 + dc) as usize))
        .filter(move |&(a, b)| a < n && b < m)
}

pub fn neighbors4(r: usize, c: usize, n: usize, m: usize) -> impl Iterator<Item = (usize, usize)> {
    neighbors_iter(&DIR4, r, c, n, m)
}

pub fn neighbors8(r: usize, c: usize, n: usize, m: usize) -> impl Iterator<Item = (usize, usize)> {
    neighbors_iter(&DIR8, r, c, n, m)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (2178, 1978));
    }
}
