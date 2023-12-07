use std::str::FromStr;


fn parse_int_list<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.split_ascii_whitespace()
        .map(|num| num.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

fn parse_spread_int<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    vec![s.replace(' ', "").parse::<T>().unwrap()]
}

pub fn run(input: String) -> Option<(String, String)> {
    let lines = input.lines().collect::<Vec<&str>>();
    // part 1

    let parsers = &[parse_int_list, parse_spread_int];

    let solutions: Vec<u64> = parsers.into_iter().map(|parser| {
        let times: Vec<u64> = parser(lines[0].split(':').last().unwrap());
        let dists: Vec<u64> = parser(lines[1].split(':').last().unwrap());
        let mut result = 1;
        for i in 0..times.len() {
            let time = times[i];
            let dist = dists[i];

            let ftime = time as f32;

            let det = (ftime * ftime - 4.0 * dist as f32).sqrt();
            let lower_bound = ((ftime - det) / 2.0).max(0.0) as u64;
            let upper_bound = ((ftime + det) / 2.0) as u64;
            println!(
                "  {} {} {}",
                lower_bound,
                upper_bound,
                upper_bound - lower_bound
            );
            result *= upper_bound - lower_bound;
        }

        result
    }).collect();

    // let part1 = {
    //     let times = parse_int_list::<u64>(lines[0].split(':').last().unwrap());
    //     let dists = parse_int_list::<u64>(lines[1].split(':').last().unwrap());

    //     let mut result = 1;
    //     for i in 0..times.len() {
    //         let time = times[i];
    //         let dist = dists[i];

    //         let ftime = time as f32;

    //         let det = (ftime * ftime - 4.0 * dist as f32).sqrt();
    //         let lower_bound = ((ftime - det) / 2.0).max(0.0) as u64;
    //         let upper_bound = ((ftime + det) / 2.0) as u64;
    //         println!(
    //             "  {} {} {}",
    //             lower_bound,
    //             upper_bound,
    //             upper_bound - lower_bound
    //         );
    //         result *= upper_bound - lower_bound;
    //     }

    //     result
    //     // println!("day 6 part 1: {result}");
    // };

    // let part2 = {
    //     let time = parse_spread_int::<i64>(lines[0].split(':').last().unwrap());
    //     let dist = parse_spread_int::<i64>(lines[1].split(':').last().unwrap());

    //     let ftime = time as f32;

    //     let det = (ftime * ftime - 4.0 * dist as f32).sqrt();
    //     let lower_bound = ((ftime - det) / 2.0).max(0.0) as u64;
    //     let upper_bound = ((ftime + det) / 2.0) as u64;

    //     // result *= upper_bound - lower_bound;

    //     // println!("{} {}", upper_bound, lower_bound);
    //     upper_bound - lower_bound;
    // };
    Some((solutions[0].to_string(), solutions[1].to_string()))
}
