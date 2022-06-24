use std::collections::HashMap;

use crate::{
    arr2::Arr2,
    parsing::{re_parser, Gather},
};

pub fn run() -> (usize, usize) {
    let input = include_str!("../input/20.txt");
    let tiles: HashMap<usize, Tile> = input
        .split("\n\n")
        .map(|blob| {
            let mut lines = blob.lines();
            let id = re_parser(r"Tile (.*):")(lines.gather());
            let tile = Arr2::from_raw(10, 10, lines.flat_map(|l| l.chars()).collect());
            (id, tile)
        })
        .collect();
    let edge_map: HashMap<Edge, Vec<usize>> = {
        let mut edge_map = HashMap::new();
        for (&id, tile) in &tiles {
            for edge in edges(tile) {
                edge_map.entry(canonical(&edge)).or_insert(vec![]).push(id);
            }
        }
        edge_map
    };
    let corner_ids: Vec<usize> = tiles
        .keys()
        .filter(|&id| {
            edges(&tiles[id])
                .into_iter()
                .filter(|edge| edge_map[&canonical(edge)].len() == 1)
                .count()
                == 2
        })
        .copied()
        .collect();
    let part1 = corner_ids.iter().product();
    let start_id = corner_ids[0];
    let start_corner = rots(&tiles[&start_id])
        .find(|tile| {
            let edges = edges(tile);
            edge_map[&canonical(&edges[0])].len() == 1 && edge_map[&canonical(&edges[3])].len() == 1
        })
        .unwrap();
    let mut tile_grid: Vec<Vec<(usize, Tile)>> = vec![vec![(start_id, start_corner)]];
    loop {
        let row = tile_grid.last_mut().unwrap();
        if let Some(entry) = find_match(row.last().unwrap(), 1, &tiles, &edge_map) {
            row.push(entry);
        } else if let Some(entry) = find_match(row.first().unwrap(), 2, &tiles, &edge_map) {
            tile_grid.push(vec![entry]);
        } else {
            break;
        }
    }
    dbg!(tile_grid.len(), tile_grid[0].len());
    let map = {
        let n = tile_grid.len();
        let m = tile_grid[0].len();
        assert!(tile_grid.iter().all(|v| v.len() == m));
        Arr2::from_fn(n * 8, m * 8, |i, j| {
            tile_grid[i / 8][j / 8].1[i % 8 + 1][j % 8 + 1]
        })
    };
    let monster = Arr2::from_raw(
        3,
        20,
        "                  # #    ##    ##    ### #  #  #  #  #  #   "
            .chars()
            .collect(),
    );
    let mut monster_squares = 0;
    for mon in versions(&monster) {
        let (n, m) = map.dims();
        let (k, l) = mon.dims();
        for r in 0..n - k {
            for s in 0..m - l {
                let good = (|| {
                    for i in 0..k {
                        for j in 0..l {
                            if mon[i][j] == '#' && map[r + i][s + j] != '#' {
                                return false;
                            }
                        }
                    }
                    true
                })();
                if good {
                    monster_squares += 15;
                }
            }
        }
    }
    let part2 = dbg!(map.into_iter().filter(|&c| c == '#').count() - monster_squares);
    (part1, part2)
}

fn find_match(
    (id, tile): &(usize, Tile),
    edge_ix: usize,
    tiles: &HashMap<usize, Tile>,
    edge_map: &HashMap<Edge, Vec<usize>>,
) -> Option<(usize, Tile)> {
    let edge = &edges(tile)[edge_ix];
    edge_map[&canonical(edge)]
        .iter()
        .find(|&k| k != id)
        .map(|&other| {
            (
                other,
                versions(&tiles[&other])
                    .find(|t| &edges(t)[(edge_ix + 2) % 4] == edge)
                    .unwrap(),
            )
        })
}

type Edge = Vec<char>;
type Tile = Arr2<char>;

fn canonical(edge: &Edge) -> Edge {
    let mut rev = edge.clone();
    rev.reverse();
    assert!(edge != &rev);
    rev.min(edge.clone())
}

fn edges(a: &Tile) -> [Edge; 4] {
    let (n, m) = a.dims();
    [
        a[0].to_vec(),
        a.col(m - 1).copied().collect(),
        a[n - 1].to_vec(),
        a.col(0).copied().collect(),
    ]
}
fn rot(a: &Tile) -> Tile {
    let (n, m) = a.dims();
    Arr2::from_fn(m, n, |i, j| a[n - j - 1][i])
}
fn rots(a: &Tile) -> impl Iterator<Item = Tile> {
    [a.clone(), rot(a), rot(&rot(a)), rot(&rot(&rot(a)))].into_iter()
}

fn flip(a: &Tile) -> Tile {
    let (n, m) = a.dims();
    Arr2::from_fn(n, m, |i, j| a[n - i - 1][j])
}
fn versions(a: &Tile) -> impl Iterator<Item = Tile> {
    rots(a).chain(rots(&flip(a)))
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (17_148_689_442_341, 2009));
    }
}
