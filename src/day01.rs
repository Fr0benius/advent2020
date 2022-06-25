use itertools::Itertools;
pub fn run() -> (i64, i64) {
    let input = include_str!("../input/01.txt");
    let nums: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();
    let part1 = find_product(&nums, 2);
    println!("Part 1: {}", part1);
    let part2 = find_product(&nums, 3);
    println!("Part 2: {}", part2);
    (part1, part2)
}

fn find_product(nums: &[i64], k: usize) -> i64 {
    nums.iter()
        .combinations(k)
        .find(|v| v.iter().copied().sum::<i64>() == 2020)
        .map(|v| v.iter().copied().product())
        .unwrap()
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (997_899, 131_248_694));
    }
}
