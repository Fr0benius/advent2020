use std::collections::{HashMap, HashSet};

use crate::{
    matching::BipartiteMatcher,
    parsing::re_parser,
};

pub fn run() -> (usize, String) {
    let input = include_str!("../input/21.txt");
    let parser = re_parser(r"(.*) \(contains (.*)\)");
    let recipes: Vec<(Vec<&str>, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let (ing, aller): (&str, &str) = parser(line);
            (ing.split(' ').collect(), aller.split(", ").collect())
        })
        .collect();

    #[allow(clippy::type_complexity)]
    let (ing_ids, ing_list, aller_ids, aller_list): (
        HashMap<&str, usize>,
        Vec<&str>,
        HashMap<&str, usize>,
        Vec<&str>,
    ) = {
        let mut ings: Vec<&str> = recipes.iter().flat_map(|v| &v.0).copied().collect();
        let mut allers: Vec<&str> = recipes.iter().flat_map(|v| &v.1).copied().collect();
        ings.sort_unstable();
        ings.dedup();
        allers.sort_unstable();
        allers.dedup();
        let assign = |ls: &Vec<_>| ls.iter().enumerate().map(|(i, &s)| (s, i)).collect();
        (assign(&ings), ings, assign(&allers), allers)
    };
    let possibilities = {
        let all_ings: HashSet<&str> = ing_ids.keys().copied().collect();
        let mut map: HashMap<&str, HashSet<&str>> = aller_ids
            .keys()
            .map(|&aller| (aller, all_ings.clone()))
            .collect();
        for (ings, allers) in &recipes {
            let ing_set = ings.iter().copied().collect();
            for aller in allers {
                let entry = map.entry(aller).or_default();
                *entry = entry.intersection(&ing_set).copied().collect();
            }
        }
        map
    };
    let (aller_to_ing, ing_to_aller, _) = {
        let n = aller_ids.len();
        let m = ing_ids.len();
        let g = (0..n)
            .map(|i| {
                possibilities[&aller_list[i]]
                    .iter()
                    .map(|&ing| ing_ids[&ing])
                    .collect()
            })
            .collect();
        let matcher = BipartiteMatcher::new(n, m, g);
        matcher.matching()
    };
    let unmatched_ings: HashSet<&str> = ing_to_aller
        .iter()
        .enumerate()
        .filter_map(|(i, aller)| {
            if aller.is_none() {
                Some(ing_list[i])
            } else {
                None
            }
        })
        .collect();
    let part1 = recipes
        .iter()
        .map(|(ings, _)| {
            ings.iter()
                .map(|ing| unmatched_ings.contains(ing) as usize)
                .sum::<usize>()
        })
        .sum();
    let part2 = aller_list
        .iter()
        .filter_map(|aller| aller_to_ing[aller_ids[aller]].map(|ing| ing_list[ing]))
        .collect::<Vec<_>>()
        .join(",");
    (part1, part2)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(
            super::run(),
            (
                2786,
                "prxmdlz,ncjv,knprxg,lxjtns,vzzz,clg,cxfz,qdfpq".into()
            )
        );
    }
}
