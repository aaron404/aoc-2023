use std::collections::HashSet;

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

pub fn run(input: String) -> Option<(String, String)> {
    let grid = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<&[u8]>>();

    let (width, height) = (grid[0].len(), grid.len());

    let mut part_numbers = Vec::new();
    for _ in 0..height {
        part_numbers.push(vec![None; width]);
    }

    let mut part1_sum = 0;

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
                    part1_sum += num;
                }
                col = i;
            }
            col += 1;
        }
    }

    let mut part2_sum = 0;
    for row in 0..height {
        for col in 0..width {
            if grid[row][col] == b'*' {
                part2_sum += calc_gear_ratio(&part_numbers, row, col);
            }
        }
    }

    Some((part1_sum.to_string(), part2_sum.to_string()))
}
