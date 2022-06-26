pub fn run() -> (usize, usize) {
    let input = include_str!("../input/23.txt");
    let init: Vec<usize> = input
        .trim()
        .bytes()
        .map(|c| (c - b'0') as usize)
        .collect();

    let part1 = simulate(&init, 100)
        .iter()
        .skip(1)
        .map(|&k| (b'0' + k as u8) as char)
        .collect::<String>()
        .parse()
        .unwrap();
    let n = init.len();
    let extended: Vec<usize> = init.into_iter().chain(n + 1..=1_000_000).collect();
    let res = simulate(&extended, 10_000_000);
    let part2 = res[1] * res[2];
    (part1, part2)
}

fn simulate(init: &[usize], times: usize) -> Vec<usize> {
    let n = init.len();
    let mut cur = init[0];
    let mut prv = vec![0; n + 1];
    let mut nxt = vec![0; n + 1];
    for w in init.windows(2) {
        prv[w[1]] = w[0];
        nxt[w[0]] = w[1];
    }
    nxt[init[n - 1]] = init[0];
    prv[init[0]] = init[n - 1];
    for _ in 0..times {
        let a = nxt[cur];
        let b = nxt[a];
        let c = nxt[b];
        {
            let y = nxt[c];
            nxt[cur] = y;
            prv[y] = cur;
        }
        {
            let mut x = (cur + n - 2) % n + 1;
            while [a, b, c].contains(&x) {
                x = (x + n - 2) % n + 1;
            }
            let y = nxt[x];
            nxt[x] = a;
            prv[a] = x;
            nxt[c] = y;
            prv[y] = c;
        }
        cur = nxt[cur];
    }
    let mut x = 1;
    let mut res = vec![];
    while res.len() < n {
        res.push(x);
        x = nxt[x];
    }
    res
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn answer_correct() {
        assert_eq!(super::run(), (32_897_654, 186_715_244_496));
    }
}
