use std::collections::{HashMap, HashSet};

use crate::parsing::{new_re, parse_re};

pub fn run() -> (usize, i64) {
    let input = include_str!("../input/07.txt");
    // let input = include_str!("../input/sample07.txt");
    let re_line = new_re(r#"(.*) bags contain (.*)\."#);
    let re = new_re(r#"([^ ]*) (.*) bags?"#);
    let parsed: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (desc, insides): (&str, &str) = parse_re(&re_line, line);
            let parsed_insides: Vec<(i64, &str)> = if insides.starts_with("no") {
                vec![]
            } else {
                insides.split(", ").map(|s| parse_re(&re, s)).collect()
            };
            (desc, parsed_insides)
        }).collect();

    let mut seen: HashSet<&str> = HashSet::new();
    let mut golden: HashSet<&str> = HashSet::new();
    let mut count: HashMap<&str, i64> = HashMap::new();
    for &bag in parsed.keys() {
        dfs(bag, &parsed, &mut seen, &mut golden, &mut count);
    }
    let part1 = golden.len() - 1;
    println!("Part 1: {part1}");
    let part2 = count["shiny gold"] - 1;
    println!("Part 2: {part2}");
    (part1, part2)
}

fn dfs<'a>(v: &'a str, graph: &HashMap<&'a str, Vec<(i64, &'a str)>>, seen: &mut HashSet<&'a str>, golden: &mut HashSet<&'a str>, count: &mut HashMap<&'a str, i64>) {
    if !seen.insert(v) {
        return;
    }
    let mut res = 1;
    for &(k, w) in &graph[v] {
        dfs(w, graph, seen, golden, count);
        res += k * count[w];
        if golden.contains(w) {
            golden.insert(v);
        }
    }
    if v == "shiny gold" {
        golden.insert(v);
    }
    count.insert(v, res);
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (115, 1250));
    }
}
