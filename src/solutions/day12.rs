#[derive(Clone, Copy, Debug, PartialEq)]
enum Condition {
    Good,
    Damaged,
    Unknown,
}

impl Condition {
    fn to_char(&self) -> char {
        let c = match self {
            Condition::Good => b'.',
            Condition::Damaged => b'#',
            Condition::Unknown => b'?',
        };
        c as char
    }
}

#[derive(Clone)]
struct Report {
    groups: Vec<u8>,
    conditions: Vec<Condition>,
}

impl Report {
    fn count_variations(&self) -> u32 {
        if let Some(n) = self
            .conditions
            .iter()
            .position(|&e| e == Condition::Unknown)
        {
            let good = self.set_condition(n, Condition::Good);
            let bad = self.set_condition(n, Condition::Damaged);
            return good.count_variations() + bad.count_variations();
        } else {
            if self.is_valid() {
                1
            } else {
                0
            }
        }
    }

    fn count_variations2(&self) -> u32 {
        let fwd_groups = self.get_groups(true);
        let bwd_groups = self.get_groups(false);

        self.print();
        println!("  {:?} {:?}", fwd_groups, bwd_groups);
        println!();

        todo!()
    }

    fn get_groups(&self, fwd: bool) -> (Vec<u8>, Option<usize>) {
        let mut groups = Vec::new();

        let mut current_bad_count = 0;
        let mut current_state = Condition::Unknown;

        let conditions = if fwd {
            self.conditions.clone()
        } else {
            let mut c = self.conditions.clone();
            c.reverse();
            c
        };

        for (i, condition) in conditions.iter().enumerate() {
            match condition {
                Condition::Good => {
                    if current_state == Condition::Damaged {
                        groups.push(current_bad_count);
                        current_state = Condition::Good;
                        current_bad_count = 0;
                    }
                }
                Condition::Damaged => {
                    current_bad_count += 1;
                    current_state = Condition::Damaged;
                }
                Condition::Unknown => {
                    let mut unknown_count = 1;
                    let mut j = i + 1;
                    while j < self.conditions.len() && self.conditions[j] == Condition::Unknown {
                        unknown_count += 1;
                        j += 1;
                    }
                    return (groups, Some(unknown_count));
                }
            }
        }

        if current_bad_count > 0 {
            groups.push(current_bad_count);
        }

        (groups, None)
    }

    fn count_variations3(&self) -> u32 {
        let unknown_mask = self.get_mask(Condition::Unknown);
        let damaged_mask = self.get_mask(Condition::Damaged);

        println!("u {unknown_mask:0>80b}");
        println!("d {damaged_mask:0>80b}");

        todo!()
    }

    fn get_mask(&self, cond: Condition) -> u128 {
        self.conditions
            .iter()
            .enumerate()
            .filter(|(i, c)| **c == cond)
            .map(|(i, c)| 1u128 << i)
            .fold(0, |acc, e| acc | e)
    }

    fn set_condition(&self, index: usize, condition: Condition) -> Self {
        let mut copy = self.clone();
        copy.conditions[index] = condition;
        copy
    }

    fn is_valid(&self) -> bool {
        let mut clues = Vec::new();
        let mut current_bad_count = 0;
        let mut current_state = Condition::Unknown;
        for condition in self.conditions.iter() {
            match condition {
                Condition::Good => {
                    if current_state == Condition::Damaged {
                        clues.push(current_bad_count);
                        current_state = Condition::Good;
                        current_bad_count = 0;
                    }
                }
                Condition::Damaged => {
                    current_bad_count += 1;
                    current_state = Condition::Damaged;
                }
                Condition::Unknown => return false,
            }
        }

        if current_bad_count > 0 {
            clues.push(current_bad_count);
        }

        clues == self.groups
    }

    fn print(&self) {
        self.conditions
            .iter()
            .for_each(|c| print!("{}", c.to_char()));
        print!(": {:?}", self.groups);
    }
}

fn byte_to_condition(b: &u8) -> Condition {
    use Condition::*;
    match b {
        b'#' => Damaged,
        b'.' => Good,
        b'?' => Unknown,
        _ => panic!("invalid state"),
    }
}

pub fn run(input: String) -> Option<(String, String)> {
    let reports = input
        .lines()
        .map(|line| {
            let mut split = line.split_ascii_whitespace();
            let conditions = split
                .next()
                .unwrap()
                .repeat(5)
                .as_bytes()
                .iter()
                .map(|b| byte_to_condition(b))
                .collect();

            let groups = split
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
                .repeat(5);

            Report { groups, conditions }
        })
        .collect::<Vec<Report>>();

    println!();

    let part1: u32 = reports
        .iter()
        .take(1)
        .enumerate()
        .map(|(i, report)| {
            print!("{i} / {}:   ", reports.len());
            report.print();
            println!();
            report.count_variations3()
        })
        .sum();
    let part2 = "";
    Some((part1.to_string(), part2.to_string()))
}
