use std::collections::HashMap;

use crate::parsing::{new_re, parse_re};

pub fn run() -> (i64, i64) {
    let input = include_str!("../input/14.txt");
    let mut mem = [HashMap::new(), HashMap::new()];
    let mask_re = new_re(r"mask = (.*)");
    let mem_re = new_re(r"mem\[(.*)\] = (.*)");
    let mut mask = "";
    for line in input.lines() {
        if mask_re.is_match(line) {
            mask = parse_re(&mask_re, line);
        } else {
            let (addr, val): (i64, i64) = parse_re(&mem_re, line);
            let val2 = apply_mask_val(val, mask);
            let addr2 = apply_mask_addr(addr, mask);
            mem[0].insert(addr, val2);
            for a in addr2 {
                mem[1].insert(a, val);
            }
        }
    }
    let part1: i64 = mem[0].values().sum();
    let part2: i64 = mem[1].values().sum();
    (part1, part2)
}

fn apply_mask_val(mut n: i64, mask: &str) -> i64 {
    for (i, c) in mask.chars().enumerate() {
        let k = 35 - i;
        match c {
            '1' => n |= 1<<k,
            '0' => n &= !(1<<k),
            _=> (),
        }
    }
    n
}

fn apply_mask_addr(mut n: i64, mask: &str) -> Vec<i64> {
    let mut fl = vec![];
    for (i, c) in mask.chars().enumerate() {
        let k = 35 - i;
        match c {
            '1' => n |= 1<<k,
            'X' => fl.push(k),
            _=> (),
        }
    }
    let mut res = vec![n];
    for j in fl {
        let len = res.len();
        for i in 0..len {
            res.push(res[i] ^ (1 << j));
        }
    }
    res
}


#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (9_615_006_043_476, 4_275_496_544_925));
    }
}
