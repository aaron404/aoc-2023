fn parse_int_list(s: &str) -> Vec<i64> {
    s.split_ascii_whitespace()
        .map(|num| num.parse::<i64>().unwrap())
        .collect()
}

fn predict(seq: Vec<i64>) -> (i64, i64) {
    let mut nth_difs: Vec<Vec<i64>> = Vec::new();
    nth_difs.push(seq);

    loop {
        let difs = nth_difs
            .last()
            .unwrap()
            .array_windows()
            .map(|window: &[i64; 2]| window[1] - window[0])
            .collect::<Vec<i64>>();
        if difs.iter().all(|&v| v == 0) {
            let mut dif = 0i64;
            let mut prev_pred = 0i64;
            nth_difs.iter().rev().for_each(|ds| {
                dif += ds.last().unwrap();
                prev_pred = ds.first().unwrap() - prev_pred;
            });
            return (dif, prev_pred);
        } else {
            nth_difs.push(difs);
        }
    }
}

pub fn run(input: String) -> Option<(String, String)> {
    let lines = input.lines().map(parse_int_list).collect::<Vec<Vec<i64>>>();
    let mut part1 = 0;
    let mut part2 = 0;
    for line in lines {
        let (next, prev) = predict(line);
        part1 += next;
        part2 += prev;
    }

    return Some((part1.to_string(), part2.to_string()));
}
