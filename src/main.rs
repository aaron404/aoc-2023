#![feature(iter_array_chunks)]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]

use std::{collections::HashSet, str::FromStr};

const NUMBERS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn line_to_num(line: &str) -> u32 {
    let digits = line
        .chars()
        .filter(|c| "0123456789".contains(*c))
        .collect::<Vec<char>>();
    let first = digits.first().unwrap().to_string().parse::<u32>().unwrap();
    let last = digits.last().unwrap().to_string().parse::<u32>().unwrap();
    first * 10 + last
}

fn line_to_sum2(line: &str) -> u32 {
    let mut finds = Vec::new();

    for i in 0..line.len() {
        if let Ok(n) = line.to_string()[i..i + 1].parse::<u32>() {
            finds.push(n)
        } else {
            for (j, num) in NUMBERS.iter().enumerate() {
                if line[i..].starts_with(num) {
                    finds.push(j as u32);
                }
            }
        }
    }
    let first = finds.first().unwrap();
    let last = finds.last().unwrap();
    first * 10 + last
}

fn day2() {
    let input = include_str!("input/input2.txt");
    let maxs = [12, 13, 14];
    let mut sum = 0;
    let mut power_sum = 0;

    for (game, line) in input.lines().enumerate() {
        let mut min_required = [0, 0, 0];
        let mut counts = [0, 0, 0];
        line.split(':').last().unwrap().split(';').for_each(|game| {
            game.split_ascii_whitespace()
                .array_chunks::<2>()
                .for_each(|chunk| {
                    let count = chunk[0].parse::<u32>().unwrap();
                    match chunk[1].as_bytes()[0] {
                        b'r' => {
                            counts[0] = counts[0].max(count);
                            min_required[0] = min_required[0].max(count);
                        }
                        b'g' => {
                            counts[1] = counts[1].max(count);
                            min_required[1] = min_required[1].max(count);
                        }
                        b'b' => {
                            counts[2] = counts[2].max(count);
                            min_required[2] = min_required[2].max(count);
                        }
                        _ => (),
                    }
                });
        });

        if counts
            .into_iter()
            .zip(maxs)
            .all(|(count, max)| count <= max)
        {
            sum += game + 1;
        }

        let power = min_required.into_iter().reduce(|acc, e| acc * e).unwrap();
        power_sum += power;
    }

    println!("sum: {sum}");
    println!("power_sum: {power_sum}");
}

fn is_symbol(byte: u8) -> bool {
    (byte != b'.') && !(48..=57).contains(&byte)
}

fn byte_to_digit(byte: u8) -> u32 {
    if !byte.is_ascii_digit() {
        panic!();
    } else {
        (byte - b'0') as u32
    }
}

fn is_part_num(grid: &Vec<&[u8]>, row: usize, col: usize, len: usize) -> bool {
    let x1 = col.saturating_sub(1);
    let x2 = (col + len).min(grid[0].len());
    let y1 = row.saturating_sub(1);
    let y2 = (row + 1).min(grid.len());

    for y in y1..=y2 {
        for x in x1..=x2 {
            if y < grid.len() && x < grid[0].len() && is_symbol(grid[y][x]) {
                return true;
            }
        }
    }

    false
}

fn calc_gear_ratio(part_nums: &Vec<Vec<Option<u32>>>, row: usize, col: usize) -> u32 {
    let width = part_nums[0].len();
    let height = part_nums.len();

    let mut part_num_count = 0;
    let mut seen_part_nums = HashSet::new();
    let mut gear_ratio = 1;

    let mut parts = Vec::new();

    for y in -1..=1 {
        for x in -1..=1 {
            let r = row.wrapping_add_signed(y);
            let c = col.wrapping_add_signed(x);

            if r < height && c < width {
                if let Some(num) = part_nums[r][c] {
                    if seen_part_nums.insert(num) {
                        part_num_count += 1;
                        gear_ratio *= num;
                        parts.push(num);
                    }
                } else {
                    seen_part_nums.clear();
                }
            }
        }
    }

    if part_num_count == 2 {
        gear_ratio
    } else {
        0
    }
}

fn day3() {
    let input = include_str!("input/input3.txt");
    let grid = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();

    let (width, height) = (grid[0].len(), grid.len());

    let mut part_numbers = Vec::new();
    for _ in 0..height {
        part_numbers.push(vec![None; width]);
    }

    let mut sum = 0;

    for row in 0..height {
        let mut col = 0;
        while col < width {
            let c = grid[row][col];
            if c.is_ascii_digit() {
                let mut num: u32 = byte_to_digit(c);
                let mut i = col + 1;
                while i < width && grid[row][i].is_ascii_digit() {
                    num = num * 10 + byte_to_digit(grid[row][i]);
                    i += 1;
                }
                let num_length = i - col;
                if is_part_num(&grid, row, col, num_length) {
                    for x in col..col + num_length {
                        part_numbers[row][x] = Some(num);
                    }
                    sum += num;
                }
                col = i;
            }
            col += 1;
        }
    }

    println!("day 3 part 1: {sum}");

    sum = 0;
    for row in 0..height {
        for col in 0..width {
            if grid[row][col] == b'*' {
                sum += calc_gear_ratio(&part_numbers, row, col);
            }
        }
    }
    println!("day 3 part 2: {sum}");
}

struct Card {
    winning_nums: HashSet<u8>,
    given_nums: Vec<u8>,
    hits: u8,
    id: u32,
}

impl Card {
    fn parse(line: &str) -> Self {
        let mut winning_nums = HashSet::new();
        let mut given_nums = Vec::new();
        let mut hits = 0;
        let mut sections = line.split('|');
        let mut first_half = sections.next().unwrap().split(':');
        let id = first_half
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        first_half
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u8>().unwrap())
            .for_each(|num| {
                winning_nums.insert(num);
            });
        sections
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u8>().unwrap())
            .for_each(|num| {
                given_nums.push(num);
                if winning_nums.contains(&num) {
                    hits += 1;
                }
            });

        Self {
            winning_nums,
            given_nums,
            hits,
            id,
        }
    }

    fn score(&self) -> u32 {
        match self.hits {
            0 => 0,
            n => 1 << (n - 1),
        }
    }
}

fn day4() {
    let input = include_str!("input/input4.txt");

    let mut total_score = 0;
    input.lines().for_each(|line| {
        let game = Card::parse(line);
        total_score += game.score();
    });

    println!("day 4 part1 score: {total_score}");

    let cards = input.lines().map(Card::parse).collect::<Vec<Card>>();
    let mut counts = vec![1; cards.len()];

    for i in 0..cards.len() {
        let count = counts[i];
        let num_hits = cards[i].hits;
        for j in 0..num_hits as usize {
            counts[i + j + 1] += count;
        }
    }

    println!("day 4 part2 count: {}", counts.into_iter().sum::<u32>());
}

#[derive(Debug)]
struct Range {
    src: u64,
    dst: u64,
    len: u64,
}

impl Range {
    fn map(&self, num: u64) -> Option<u64> {
        if num >= self.src && num < self.src + self.len {
            return Some(num - self.src + self.dst);
        }
        None
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
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
}

fn day5() {
    let input = include_str!("input/input5.txt");
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
                current_map.ranges.push(Range {
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

    // println!("{seeds:?}");
    // println!("{maps:?}");

    let mut best_location = u64::MAX;
    for seed in seeds.iter() {
        let mut val = *seed;
        // println!("mapping seed: {seed}");
        for map in maps.iter() {
            val = map.map(val);
            // println!("  {val}");
        }
        if val < best_location {
            best_location = val;
        }
    }
    println!("day 5 part 1: {best_location}");

    let mut best_location = u64::MAX;
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
            if val < best_location {
                best_location = val;
                println!("new best: {best_location}");
            }
        }
    }

    println!("day 5 part 2: {best_location}");
}

fn parse_int_list<T>(s: &str) -> Vec<T>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.split_ascii_whitespace()
        .map(|num| num.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}

fn parse_spread_int<T>(s: &str) -> T
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    s.replace(' ', "").parse::<T>().unwrap()
}

fn day6() {
    let input = include_str!("input/test6.txt");
    let lines = input.lines().collect::<Vec<&str>>();
    // part 1
    {
        let times = parse_int_list::<u64>(lines[0].split(':').last().unwrap());
        let dists = parse_int_list::<u64>(lines[1].split(':').last().unwrap());

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

        println!("day 6 part 1: {result}");
    }

    // part 2
    {
        let time = 7; //parse_spread_int::<i64>(lines[0].split(':').last().unwrap());
        let dist = 9; //parse_spread_int::<i64>(lines[1].split(':').last().unwrap());

        let ftime = time as f32;

        let det = (ftime * ftime - 4.0 * dist as f32).sqrt();
        let lower_bound = ((ftime - det) / 2.0).max(0.0) as u64;
        let upper_bound = ((ftime + det) / 2.0) as u64;

        // result *= upper_bound - lower_bound;

        println!("{} {}", upper_bound, lower_bound);
    }
}

fn main() {
    // let input = include_str!("input/input1.txt");
    // let result: u32 = input.lines().map(line_to_num).sum();
    // let result2: u32 = input.lines().map(line_to_sum2).sum();
    // println!("{result}");
    // println!("{result2}");

    // day2();
    // day3();
    // day4();
    // day5();
    day6();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
