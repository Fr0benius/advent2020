pub fn run() -> (i64, i64) {
    let input = include_str!("../input/05.txt");
    let mut ids: Vec<i64> = input
        .lines()
        .map(|line| {
            line.chars().fold((0, 0), |(a, b), c| match c {
                'F' => (2 * a, b),
                'B' => (2 * a + 1, b),
                'L' => (a, 2 * b),
                'R' => (a, 2 * b + 1),
                _ => unreachable!(),
            })
        })
        .map(|(a, b)| a * 8 + b)
        .collect();
    let part1 = *ids.iter().max().unwrap();

    println!("Part 1: {part1}");
    ids.sort_unstable();
    let part2;
    for i in 0.. {
        if ids[i + 1] - ids[i] > 1 {
            part2 = ids[i] + 1;
            println!("Part 2: {part2}");
            return (part1, part2);
        }
    }
    (0, 0)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (871, 640));
    }
}
