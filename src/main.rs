#![feature(iter_array_chunks)]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(array_windows)]

use std::{fs::File, io::Read};

mod solutions;

fn main() -> std::io::Result<()> {
    let mode = if std::env::args().nth(1).is_some() {
        println!("Solutions (test input)");
        "test"
    } else {
        println!("Solutions (real input)");
        "input"
    };

    for (i, solution) in solutions::SOLUTIONS.iter().enumerate() {
        let day = i + 1;

        let fname = format!("input/{mode}{day}.txt");
        let mut buffer = String::new();
        File::open(&fname)
            .unwrap_or_else(|_| panic!("{}", format!("Failed to open {fname}")))
            .read_to_string(&mut buffer)?;
        print!("Day {day: >2}: ");
        if [5].contains(&day) {
            println!("     -------- skipped --------");
            continue;
        }
        if let Some((part1, part2)) = solution(buffer) {
            println!("{part1: >16}{part2: >16}");
        }
    }

    Ok(())
}
