use crate::parsing::parse_iter;

pub fn run() -> (i32, i64) {
    let mut nums: Vec<i64> = parse_iter(&mut include_str!("../input/10.txt").lines());
    nums.push(0);
    nums.sort_unstable();
    nums.push(nums.last().unwrap() + 3);
    let mut diffs = [0; 4];
    for w in nums.windows(2) {
        diffs[(w[1] - w[0]) as usize] += 1;
    }
    let part1 = dbg!(diffs[1] * diffs[3]);
    let n = nums.len();
    let mut dp = vec![0i64; n];
    dp[0] = 1;
    for i in 1..n {
        for j in (0..i).rev() {
            if nums[i] - nums[j] > 3 {
                break;
            }
            dp[i] += dp[j];
        }
    }
    let part2 = dbg!(dp[n - 1]);
    (part1, part2)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (2470, 1973822685184));
    }
}
