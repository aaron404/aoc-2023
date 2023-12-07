use std::{cmp::Ordering, collections::HashMap};

fn card_to_rank(card: &u8) -> u8 {
    match card {
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        c => panic!("got unexpected char: {c}"),
    }
}

fn card_to_rank2(card: &u8) -> u8 {
    match card {
        b'J' => 1,
        _ => card_to_rank(card),
    }
}

struct Hand {
    cards: Vec<u8>,
    bid: u32,
}

impl Hand {
    fn rank(&self) -> u8 {
        let mut counts: HashMap<u8, u8> = HashMap::new();
        for card in self.cards.iter() {
            if counts.contains_key(card) {
                *counts.get_mut(card).unwrap() += 1;
            } else {
                counts.insert(*card, 1);
            }
        }

        let mut counts = counts.values().copied().collect::<Vec<u8>>();
        counts.sort_by(|&a, &b| a.cmp(&b).reverse());
        match counts[0] {
            5 => 7,
            4 => 6,
            3 => match counts[1] {
                2 => 5,
                1 => 4,
                _ => panic!(),
            },
            2 => match counts[1] {
                2 => 3,
                1 => 2,
                _ => panic!(),
            },
            1 => 1,
            _ => panic!(),
        }
    }

    fn rank2(&self) -> u8 {
        let mut counts: HashMap<u8, u8> = HashMap::new();
        let mut joker_count = 0;
        for card in self.cards.iter() {
            if *card == b'J' {
                joker_count += 1;
                continue;
            }
            if counts.contains_key(card) {
                *counts.get_mut(card).unwrap() += 1;
            } else {
                counts.insert(*card, 1);
            }
        }

        let mut counts = counts.values().copied().collect::<Vec<u8>>();
        counts.sort_by(|&a, &b| a.cmp(&b).reverse());
        if counts.is_empty() {
            counts.push(5);
        } else {
            counts[0] += joker_count;
        }
        match counts[0] {
            5 => 7,
            4 => 6,
            3 => match counts[1] {
                2 => 5,
                1 => 4,
                _ => panic!(),
            },
            2 => match counts[1] {
                2 => 3,
                1 => 2,
                _ => panic!(),
            },
            1 => 1,
            _ => panic!(),
        }
    }

    fn check_labels(&self, other: &Self) -> Ordering {
        for (cur, other) in self
            .cards
            .iter()
            .map(card_to_rank)
            .zip(other.cards.iter().map(card_to_rank))
        {
            let ord = cur.cmp(&other);
            if ord != Ordering::Equal {
                return ord;
            }
        }

        Ordering::Equal
    }

    fn check_labels2(&self, other: &Self) -> Ordering {
        for (cur, other) in self
            .cards
            .iter()
            .map(card_to_rank2)
            .zip(other.cards.iter().map(card_to_rank2))
        {
            let ord = cur.cmp(&other);
            if ord != Ordering::Equal {
                return ord;
            }
        }

        Ordering::Equal
    }

    fn cmp(a: &Self, b: &Self) -> Ordering {
        let ord = a.rank().cmp(&b.rank());
        match ord {
            Ordering::Equal => a.check_labels(b),
            _ => ord,
        }
    }

    fn cmp2(a: &Self, b: &Self) -> Ordering {
        let ord = a.rank2().cmp(&b.rank2());
        match ord {
            Ordering::Equal => a.check_labels2(b),
            _ => ord,
        }
    }
}

pub fn run(input: String) -> Option<(String, String)> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let cards = parts.next().unwrap().as_bytes().to_vec();
            let bid: u32 = parts.next().unwrap().parse().unwrap();
            Hand { cards, bid }
        })
        .collect();

    hands.sort_by(Hand::cmp);

    let part1: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum();

    // part 2
    hands.sort_by(Hand::cmp2);

    let part2: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum();

    Some((part1.to_string(), part2.to_string()))
}
