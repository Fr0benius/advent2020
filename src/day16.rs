use crate::{parsing::{parse_iter, re_parser}, matching::BipartiteMatcher};

type Spec<'a> = (&'a str, (i64, i64), (i64, i64));
pub fn run() -> (i64, i64) {
    let input = include_str!("../input/16.txt");
    // let input = include_str!("../input/sample16.txt");
    let parts: Vec<&str> = input.split("\n\n").collect();
    let specs: Vec<Spec> = parts[0]
        .lines()
        .map(re_parser(r"(.*): (.*)-(.*) or (.*)-(.*)"))
        .collect();
    let my_ticket = parse_ticket(parts[1].lines().nth(1).unwrap());
    let tickets: Vec<_> = parts[2].lines().skip(1).map(parse_ticket).collect();

    let part1 = tickets.iter().map(|t| invalid_score(t, &specs)).sum();
    let valid_tickets: Vec<_> = tickets
        .into_iter()
        .filter(|t| t.iter().all(|&f| specs.iter().any(|&s| fits(s, f))))
        .collect();
    let n_fields = specs.len();
    let g: Vec<Vec<usize>> = (0..n_fields)
        .map(|i| {
            (0..n_fields)
                .filter(|&j| valid_tickets.iter().all(|t| fits(specs[i], t[j])))
                .collect()
        })
        .collect();
    let (mat, _, n_matches) = BipartiteMatcher::new(n_fields, n_fields, g).matching();
    assert_eq!(n_fields, n_matches);
    let part2: i64 = (0..n_fields)
        .filter_map(|i| {
            if specs[i].0.starts_with("departure") {
                mat[i]
            } else {
                None
            }
        })
        .map(|j| my_ticket[j])
        .product();
    (part1, part2)
}

fn parse_ticket(s: &str) -> Vec<i64> {
    parse_iter(&mut s.split(','))
}

fn invalid_score(ticket: &[i64], specs: &[Spec]) -> i64 {
    ticket
        .iter()
        .map(|&x| {
            if specs.iter().any(|&spec| fits(spec, x)) {
                0
            } else {
                x
            }
        })
        .sum()
}

fn fits((_, (l1, r1), (l2, r2)): Spec, x: i64) -> bool {
    (l1..=r1).contains(&x) || (l2..=r2).contains(&x)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (20048, 4_810_284_647_569));
    }
}
