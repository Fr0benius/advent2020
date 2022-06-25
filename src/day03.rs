pub fn run() -> (i64, i64) {
    let input = include_str!("../input/03.txt");
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let part1 = check_slope(1, 3, &grid);
    println!("Part 1: {part1}");
    let part2: i64 = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|&(dr, dc)| check_slope(dr, dc, &grid))
        .product();
    println!("Part 2: {part2}");
    (part1, part2)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (191, 1_478_615_040));
    }
}

fn check_slope(dr: usize, dc: usize, grid: &[Vec<char>]) -> i64 {
    let mut r = 0;
    let mut c = 0;
    let mut res = 0;
    while r < grid.len() {
        res += (grid[r][c] == '#') as i64;
        r += dr;
        c = (c + dc) % grid[0].len();
    }
    res
}
