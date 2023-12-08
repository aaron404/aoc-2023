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
    offset: u64,
}

fn find_cycle(nodes: &HashMap<u32, Node>, start: u32, dirs: &str) -> Cycle {
    let mut num_steps = 0u64;
    let mut position = start;
    // Keep track of the state each time we hit a potential end node.
    // For each end node, keep a map of the current position index and
    // the step count from the last time it was seen
    let mut states: HashMap<u32, HashMap<u16, u64>> = HashMap::new();
    for (i, dir) in dirs.as_bytes().iter().enumerate().cycle() {
        match dir {
            b'L' => position = nodes.get(&position).unwrap().left,
            b'R' => position = nodes.get(&position).unwrap().right,
            _ => panic!(),
        }
        num_steps += 1;

        if is_end_node(position) {
            if let Some(h) = states.get_mut(&position) {
                if let Some(step_count) = h.get(&(i as u16)) {
                    // found cycle!
                    return Cycle {
                        len: num_steps - step_count,
                        offset: num_steps,
                    };
                } else {
                    h.insert(i as u16, num_steps);
                }
            } else {
                let mut h = HashMap::new();
                h.insert(i as u16, num_steps);
                states.insert(position, h);
            }
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
            println!("{k:x}");
            positions.push(k);
        }
        nodes.insert(k, v);
    });

    // println!("{nodes:#x?}");

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

    dbg!(num_steps);

    let mut lcm = 1u64;
    // try to find cycle length for each position
    for position in positions.iter() {
        let cycle = find_cycle(&nodes, *position, directions);
        print!("{},", cycle.len);
        lcm = num::Integer::lcm(&lcm, &(cycle.len as u64));
    }

    let pos2cycle = |pos: &u32| find_cycle(&nodes, *pos, directions);
    let cycles = positions.iter().map(pos2cycle).collect::<Vec<Cycle>>();
    // let cycle_lens = cycles.iter().map(|cycle| cycle.len).collect::<Vec<u64>>();
    // let cycle_offsets = cycles
    // .iter()
    // .map(|cycle| cycle.offset)
    // .collect::<Vec<u64>>();

    println!("  lcm: {lcm}");

    // start first ghost at the distance travelled by the time it found its cycle
    let mut s = cycles[0].offset;
    let mut interval = cycles[0].len;
    let mut other = 1;

    // while other < cycles.len() {
    //     let o = &cycles[other];
    //     let mut num_iters = 0;
    //     while ((s - o.offset) % o.len) != 0 {
    //         s += interval;
    //         num_iters += 1;
    //     }
    //     interval = num_iters * interval;
    //     other += 1;
    // }

    loop {
        if cycles
            .iter()
            .skip(1)
            .take(5)
            .all(|cycle| ((s - cycle.offset) % cycle.len) == 0)
        {
            break;
        }
        // if ((s - cycle_offsets[1]) % cycle_lens[1]) == 0 {
        //     done = true;
        // }
        s += cycles[0].len;
    }

    println!("{s}");

    for (i, cycle) in cycles.iter().enumerate() {
        println!("{i} {}", (s - cycle.offset) / cycle.len);
    }

    // println!("num starting positions: {}", positions.len());
    // num_steps = 0u64;
    // let mut best_matches = 0;
    // for dir in directions.as_bytes().iter().cycle() {
    //     match dir {
    //         b'L' => positions
    //             .iter_mut()
    //             .for_each(|pos| *pos = nodes.get(&pos).unwrap().left), //position = nodes.get(&position).unwrap().left,
    //         b'R' => positions
    //             .iter_mut()
    //             .for_each(|pos| *pos = nodes.get(&pos).unwrap().right),
    //         _ => panic!(),
    //     }
    //     num_steps += 1;

    //     if num_steps % 100000000 == 0 {
    //         println!("{num_steps}");
    //     }

    //     let num_matches = positions
    //         .iter()
    //         .filter(|pos| (**pos as u8) & b'Z' == b'Z')
    //         .count();
    //     if num_matches > best_matches {
    //         println!("new best: {num_matches}");
    //         best_matches = num_matches;
    //     }
    //     if positions.iter().fold('Z' as u32, |acc, e| acc & e) as u8 == b'Z' {
    //         break;
    //     }
    // }

    // dbg!(num_steps);

    None
}
