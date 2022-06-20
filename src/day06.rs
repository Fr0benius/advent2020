pub fn run() -> (i64, i64) {
    let input = include_str!("../input/06.txt");
    let part1: i64 = input
        .split("\n\n")
        .map(|chunk| {
            let mut chars: Vec<_> = chunk.chars().filter(|&c| c != '\n').collect();
            chars.sort_unstable();
            chars.dedup();
            chars.len() as i64
        })
        .sum();
    println!("Part 1: {part1}");
    let part2: i64 = input
        .split("\n\n")
        .map(|chunk| {
            ('a'..='z')
                .filter(|&c|
                    chunk.lines().all(|line| line.contains(c))
                ).count() as i64
        })
        .sum();
    println!("Part 2: {part2}");
    (part1, part2)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (6947, 3398));
    }
}
