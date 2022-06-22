use crate::dir::DIR4;

pub fn run() -> (i64, i64) {
    // let ops: Vec<(char, i64)> = "F10\nN3\nF7\nR90\nF11"
    let ops: Vec<(char, i64)> = include_str!("../input/12.txt")
        .lines()
        .map(|line| {
            let (a, b) = line.split_at(1);
            (a.chars().next().unwrap(), b.parse().unwrap())
        })
        .collect();

    let (x, y, _) = ops
        .iter().fold((0, 0, 1), step);
    let part1 = dbg!(x.abs() + y.abs());
    let (x, y, _, _) = ops
        .iter().fold((0, 0, 10, 1), step2);
    let part2 = dbg!(x.abs() + y.abs());
    (part1, part2)
}

fn step((mut x, mut y, mut dir): (i64, i64, usize), &(op, amt): &(char, i64)) -> (i64, i64, usize) {
    match op {
        'N' => y += amt,
        'E' => x += amt,
        'S' => y -= amt,
        'W' => x -= amt,
        'L' => dir = (dir + 3 * (amt as usize / 90)) % 4,
        'R' => dir = (dir + amt as usize / 90) % 4,
        'F' => {
            let (dx, dy) = DIR4[dir];
            x += dx * amt;
            y += dy * amt;
        }
        _ => unreachable!(),
    }
    (x, y, dir)
}

fn rot_l(mut x: i64, mut y: i64, times: i64) -> (i64, i64) {
    for _ in 0..times {
        (x, y) = (-y, x);
    }
    (x, y)
}

fn step2((mut x, mut y, mut wx, mut wy): (i64, i64, i64, i64), &(op, amt): &(char, i64)) -> (i64, i64, i64, i64) {
    match op {
        'N' => wy += amt,
        'E' => wx += amt,
        'S' => wy -= amt,
        'W' => wx -= amt,
        'L' => (wx, wy) = rot_l(wx, wy, amt / 90),
        'R' => (wx, wy) = rot_l(wx, wy, 4 - amt / 90),
        'F' => {
            x += wx * amt;
            y += wy * amt;
        }
        _ => unreachable!(),
    }
    (x, y, wx, wy)
}


#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (1177, 46530));
    }
}
