use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    left: u32,
    right: u32,
}

fn is_start_node(key: u32) -> bool {
    (key as u8) == b'A'
}

fn is_end_node(key: u32) -> bool {
    (key as u8) == b'Z'
}

fn parse_node(s: &str) -> u32 {
    s.as_bytes()
        .into_iter()
        .map(|&c| c as u32)
        .reduce(|acc, e| ((acc << 8) + e))
        .unwrap()
}

fn parse_line(s: &str) -> (u32, Node) {
    let key = parse_node(&s[0..3]);
    let node = Node {
        left: parse_node(&s[7..10]),
        right: parse_node(&s[12..15]),
    };
    (key, node)
}

#[derive(Debug)]
struct Cycle {
    len: u64,
}

fn find_cycle(nodes: &HashMap<u32, Node>, start: u32, dirs: &str) -> Cycle {
    let mut num_steps = 0u64;
    let mut position = start;
    // Keep track of the state each time we hit a potential end node.
    // For each end node, keep a map of the current position index and
    // the step count from the last time it was seen
    for dir in dirs.as_bytes().iter().cycle() {
        match dir {
            b'L' => position = nodes.get(&position).unwrap().left,
            b'R' => position = nodes.get(&position).unwrap().right,
            _ => panic!(),
        }
        num_steps += 1;

        if is_end_node(position) {
            return Cycle { len: num_steps };
        }
    }
    panic!()
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

    let goal = parse_node("ZZZ");
    let mut position = parse_node("AAA");
    let mut num_steps = 0;

    for dir in directions.as_bytes().iter().cycle() {
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

    let mut lcm = 1u64;
    // try to find cycle length for each position
    for position in positions.iter() {
        let cycle = find_cycle(&nodes, *position, directions);
        lcm = num::Integer::lcm(&lcm, &(cycle.len as u64));
    }

    Some((num_steps.to_string(), lcm.to_string()))
}
