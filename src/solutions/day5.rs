struct Span {
    src: u64,
    len: u64,
}

#[derive(Debug)]
struct MapRange {
    src: u64,
    dst: u64,
    len: u64,
}

impl MapRange {
    fn map(&self, num: u64) -> Option<u64> {
        if self.contains(num) {
            return Some(num - self.src + self.dst);
        }
        None
    }

    fn contains(&self, num: u64) -> bool {
        if num >= self.src && num < self.src + self.len {
            return true;
        }
        false
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn map(&self, num: u64) -> u64 {
        for range in self.ranges.iter() {
            if let Some(val) = range.map(num) {
                return val;
            }
        }
        num
    }

    fn divide_span(&self, span: Span) -> (Span, Option<Span>) {
        for range in self.ranges.iter() {
            if range.contains(span.src) {
                let offset = span.src - range.src;
            }
        }
        todo!()
    }

    fn get_range(&self, spans: Vec<Span>) -> Vec<Span> {
        let mut new_spans = Vec::new();

        for span in spans {
            let mut current_span = span;
            loop {
                let (new_span, remainder) = self.divide_span(current_span);
                if let Some(s) = remainder {
                    current_span = s;
                } else {
                    break;
                }
                new_spans.push(new_span);
            }
        }

        new_spans
    }
}

pub fn run(input: String) -> Option<(String, String)> {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut maps: Vec<Map> = Vec::new();

    lines.next(); // skip first empty line
    lines.next(); // skip first map name line

    let mut current_map = Map { ranges: Vec::new() };
    let mut done = false;
    while !done {
        if let Some(line) = lines.next() {
            if line.ends_with(':') {
                maps.push(current_map);
                current_map = Map { ranges: Vec::new() };
            } else if line.is_empty() {
            } else {
                let nums = line
                    .split_ascii_whitespace()
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                current_map.ranges.push(MapRange {
                    src: nums[1],
                    dst: nums[0],
                    len: nums[2],
                });
            }
        } else {
            done = true;
        }
    }
    maps.push(current_map);

    let mut part1 = u64::MAX;
    for seed in seeds.iter() {
        let mut val = *seed;
        for map in maps.iter() {
            val = map.map(val);
        }
        if val < part1 {
            part1 = val;
        }
    }

    let mut part2 = u64::MAX;
    for i in 0..seeds.len() / 2 {
        println!("[{i}/{}]", seeds.len() / 2);
        let start = seeds[i * 2];
        let count = seeds[i * 2 + 1];

        for seed in start..start + count {
            let mut val = seed;
            // println!("mapping seed: {seed}");
            for map in maps.iter() {
                val = map.map(val);
                // println!("  {val}");
            }
            if val < part2 {
                part2 = val;
                println!("new best: {part2}");
            }
        }
    }

    Some((part1.to_string(), part2.to_string()))
}
