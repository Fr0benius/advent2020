use std::collections::HashMap;

use crate::parsing::Extract;

pub fn run() -> (i64, i64) {
    let input = include_str!("../input/09.txt");
    let nums: Vec<i64> = Extract::extract(&mut input.lines());
    let part1 = nums.windows(26).find_map(|w| {
        for i in 0..25 {
            for j in i+1..25 {
                if w[i] != w[j] && w[i] + w[j] == w[25] {
                    return None;
                }
            }
        }
        Some(w[25])
    }
    ).unwrap();
    println!("Part 1: {part1}");
    let mut s = 0;
    let mut map = HashMap::from([(0, 0)]);
    for (i, &n) in nums.iter().enumerate() {
        s += n;
        map.insert(s, i);
        if let Some(&j) = map.get(&(s - part1))  {
            let sub = &nums[j..i];
            let part2 = sub.iter().min().unwrap() + sub.iter().max().unwrap();
            println!("Part 2: {part2}");
            return (part1, part2)
        }
    }
    (0, 0)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (10884537, 1261309));
    }
}
