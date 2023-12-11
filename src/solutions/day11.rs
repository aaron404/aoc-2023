fn get_sum_of_distances(
    galaxies: &Vec<(usize, usize)>,
    expansion_x: &Vec<usize>,
    expansion_y: &Vec<usize>,
    factor: usize,
) -> usize {
    let mut sum = 0;
    for a in 0..galaxies.len() - 1 {
        for b in a + 1..galaxies.len() {
            let (mut x1, mut y1) = galaxies[a];
            let (mut x2, mut y2) = galaxies[b];
            x1 += expansion_x[x1] * factor;
            y1 += expansion_y[y1] * factor;
            x2 += expansion_x[x2] * factor;
            y2 += expansion_y[y2] * factor;
            let dist = x1.abs_diff(x2) + y1.abs_diff(y2);
            sum += dist;
        }
    }

    sum
}

pub fn run(input: String) -> Option<(String, String)> {
    let mut image = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    let width = image[0].len();
    let height = image.len();

    // find empty rows and cols
    let mut empty_cols = Vec::new();
    let mut empty_rows = Vec::new();
    for x in 0..width {
        if image.iter().all(|line| line[x] == b'.') {
            empty_cols.push(x);
        }
    }
    for y in 0..height {
        if image[y].iter().all(|&byte| byte == b'.') {
            empty_rows.push(y);
        }
    }

    let mut galaxies = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if image[y][x] == b'#' {
                galaxies.push((x, y));
            }
        }
    }

    empty_cols.push(image[0].len());
    let mut expansion_x = vec![0; image[0].len()];
    empty_cols
        .array_windows::<2>()
        .enumerate()
        .for_each(|(count, &[a, b])| {
            for i in a..b {
                expansion_x[i] = count + 1;
            }
        });

    empty_rows.push(image.len());
    let mut expansion_y = vec![0; image.len()];
    empty_rows
        .array_windows::<2>()
        .enumerate()
        .for_each(|(count, &[a, b])| {
            for i in a..b {
                expansion_y[i] = count + 1;
            }
        });

    let part1 = get_sum_of_distances(&galaxies, &expansion_x, &expansion_y, 1);
    let part2 = get_sum_of_distances(&galaxies, &expansion_x, &expansion_y, 1000000 - 1);

    Some((part1.to_string(), part2.to_string()))
}
