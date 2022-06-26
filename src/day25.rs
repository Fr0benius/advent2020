use crate::parsing::Gather;

pub fn run() -> i64 {
    const MOD: i64 = 20_201_227;
    let input = include_str!("../input/25.txt");
    let (k1, k2): (i64, i64) = input.lines().gather();
    let mut p = 1;
    let mut key = 1;
    while p != k1 {
        p = p * 7 % MOD;
        key = key * k2 % MOD;
    }
    key
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), 16_902_792);
    }
}
