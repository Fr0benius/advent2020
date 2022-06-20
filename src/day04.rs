use std::collections::HashMap;

pub fn run() -> (usize, usize) {
    let input = include_str!("../input/04.txt");
    let passports: Vec<HashMap<&str, &str>> = input
        .split("\n\n")
        .map(|line| {
            line.split_whitespace()
                .map(|kv| {
                    let mut it = kv.split(':');
                    (it.next().unwrap(), it.next().unwrap())
                })
                .collect()
        })
        .collect();
    let part1 = passports.iter().filter(|&x| all_fields(x)).count();
    println!("Part 1: {part1}");
    let part2 = passports.iter().filter(|&x| verify(x) != None).count();
    println!("Part 2: {part2}");
    (part1, part2)
}

fn all_fields(passport: &HashMap<&str, &str>) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|x| passport.contains_key(x))
}

fn verify(passport: &HashMap<&str, &str>) -> Option<()> {
    fn check_range(k: i64, l: i64, r: i64) -> Option<()> {
        if (l..=r).contains(&k) {
            Some(())
        } else {
            None
        }
    }
    check_range(passport.get("byr")?.parse().ok()?, 1920, 2002)?;
    check_range(passport.get("iyr")?.parse().ok()?, 2010, 2020)?;
    check_range(passport.get("eyr")?.parse().ok()?, 2020, 2030)?;
    {
        let hgt = passport.get("hgt")?;
        let (l, r);
        if hgt.ends_with("in") {
            (l, r) = (59, 76);
        } else if hgt.ends_with("cm") {
            (l, r) = (150, 193);
        } else {
            return None;
        }
        let h: i64 = hgt[0..hgt.len() - 2].parse().ok()?;
        check_range(h, l, r)?;
    }
    {
        let hcl = passport.get("hcl")?;

        if !(hcl.len() == 7
            && hcl.starts_with('#')
            && hcl
                .chars()
                .skip(1)
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()))
        {
            return None;
        }
    }
    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(passport.get("ecl")?) {
        return None;
    }
    {
        let &pid = passport.get("pid")?;
        if !(pid.len() == 9 && pid.chars().all(|c| c.is_digit(10))) {
            return None;
        }
    }

    Some(())
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (196, 114));
    }
}
