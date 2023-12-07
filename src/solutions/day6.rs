fn parse_int_list(s: &str) -> Vec<u64> {
    s.split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn parse_spread_int(s: &str) -> Vec<u64> {
    vec![s.replace(' ', "").parse().unwrap()]
}

pub fn run(input: String) -> Option<(String, String)> {
    let lines = input.lines().collect::<Vec<&str>>();

    // The only thing different between part 1 and 2 is the input parsing
    let parsers = &[parse_int_list, parse_spread_int];

    let solutions: Vec<u64> = parsers
        .iter()
        .map(|parser| {
            let times: Vec<u64> = parser(lines[0].split(':').last().unwrap());
            let dists: Vec<u64> = parser(lines[1].split(':').last().unwrap());
            let mut result = 1;
            for i in 0..times.len() {
                let time = times[i];
                let dist = dists[i];

                let ftime = time as f64;

                let det = (ftime * ftime - 4.0 * dist as f64).sqrt();
                let lower_bound = ((ftime - det) / 2.0).max(0.0) as u64;
                let upper_bound = ((ftime + det) / 2.0) as u64;
                result *= upper_bound - lower_bound;
            }

            result
        })
        .collect();

    Some((solutions[0].to_string(), solutions[1].to_string()))
}
