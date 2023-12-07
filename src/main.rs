#![feature(iter_array_chunks)]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
    str::FromStr,
};


mod solutions;

fn main() -> std::io::Result<()> {
    // let input = include_str!("input/input1.txt");
    // let result: u32 = input.lines().map(line_to_num).sum();
    // let result2: u32 = input.lines().map(line_to_sum2).sum();
    // println!("{result}");
    // println!("{result2}");

    // day2();
    // day3();
    // day4();
    // day5();
    // day6();
    // day7();

    let mode = if let Some(_) = std::env::args().nth(1) {
        println!("Solutions (test input)");
        "test"
    } else {
        println!("Solutions (real input)");
        "input"
    };

    for (i, solution) in solutions::SOLUTIONS.into_iter().enumerate() {
        let day = i + 1;

        let fname = format!("input/{mode}{day}.txt");
        let mut buffer = String::new();
        File::open(&fname)
            .expect(&format!("Failed to open {fname}").to_string())
            .read_to_string(&mut buffer)?;
        print!("Day {day: >2}");
        if [5, 6].contains(&day) {
            println!("       --- skipped ---");
            continue
        }
        if let Some((part1, part2)) = solution(buffer) {
            println!("{part1: >12}{part2: >12}");
        } else {
            println!(":");
        }

    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        crate::main();
    }
}
