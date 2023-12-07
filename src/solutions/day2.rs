pub fn run(input: String) -> Option<(String, String)> {
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

    Some((sum.to_string(), power_sum.to_string()))
}
