use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: u32,
    right: u32,
}

fn is_start_node(key: u32) -> bool {
    ((key & 0xff) as u8) == b'A'
}

fn parse_node(s: &str) -> u32 {
    s.as_bytes()
        .into_iter()
        .map(|&c| c as u32)
        .reduce(|acc, e| ((acc << 8) + e))
        .unwrap()
}

fn parse_line(s: &str) -> (u32, Node) {
    println!("{s}");
    let key = parse_node(&s[0..3]);
    let node = Node {
        left: parse_node(&s[7..10]),
        right: parse_node(&s[12..15]),
    };
    (key, node)
}

pub fn run(input: String) -> Option<(String, String)> {
    let mut lines = input.lines();
    let directions = lines.next().unwrap();

    lines.next();

    let mut nodes = HashMap::new();

    let mut positions = Vec::new();
    lines.map(parse_line).for_each(|(k, v)| {
        if is_start_node(k) {
            positions.push(k);
        }
        nodes.insert(k, v);
    });

    // println!("{nodes:#x?}");

    let goal = parse_node("ZZZ");
    let mut position = parse_node("AAA");
    let mut num_steps = 0;

    for (i, dir) in directions.as_bytes().iter().enumerate().cycle() {
        match dir {
            b'L' => position = nodes.get(&position).unwrap().left,
            b'R' => position = nodes.get(&position).unwrap().right,
            _ => panic!(),
        }
        num_steps += 1;

        if position == goal {
            break;
        }
    }

    dbg!(num_steps);

    println!("num starting positions: {}", positions.len());
    num_steps = 0;
    let mut best_matches = 0;
    for (i, dir) in directions.as_bytes().iter().enumerate().cycle() {
        match dir {
            b'L' => positions
                .iter_mut()
                .for_each(|pos| *pos = nodes.get(&pos).unwrap().left), //position = nodes.get(&position).unwrap().left,
            b'R' => positions
                .iter_mut()
                .for_each(|pos| *pos = nodes.get(&pos).unwrap().right),
            _ => panic!(),
        }
        num_steps += 1;

        if num_steps % 1000000 == 0 {
            // println!("{num_steps}");
        }

        let num_matches = positions
            .iter()
            .filter(|pos| (**pos as u8) & b'Z' == b'Z')
            .count();
        if num_matches > best_matches {
            println!("new best: {num_matches}");
            best_matches = num_matches;
        }
        if positions.iter().fold('Z' as u32, |acc, e| acc & e) as u8 == b'Z' {
            break;
        }
    }

    dbg!(num_steps);

    None
}
