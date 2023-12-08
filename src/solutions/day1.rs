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

pub fn run(input: String) -> Option<(String, String)> {
    // let input = include_str!("input/input1.txt");
    let part1: u32 = input.lines().map(line_to_num).sum();
    let part2: u32 = input.lines().map(line_to_sum2).sum();

    Some((part1.to_string(), part2.to_string()))
}
