use crate::{parsing::parse_iter, num::cr_theorem};

pub fn run() -> (i64, i64) {
    let (time, v): (i64, Vec<i64>) = {
        let mut lines = include_str!("../input/13.txt").lines();
        let time = parse_iter(&mut lines);
        let v = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap_or(0))
            .collect();
        (time, v)
    };
    let part1 = {
        let n = v
            .iter()
            .filter(|&&k| k != 0)
            .min_by_key(|&k| (time + k - 1) / k * k)
            .unwrap();
        n * ((time + n - 1) / n * n - time)
    };
    dbg!(part1);
    let part2 = v
        .iter()
        .enumerate()
        .filter(|&(_, &k)| k != 0)
        .fold((0, 1), |(a, n), (i, &k)| {
            (cr_theorem(a, n, -(i as i64), k), n * k)
        })
        .0;
    dbg!(part2);
    (part1, part2)
}


#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (410, 600_691_418_730_595));
    }
}
