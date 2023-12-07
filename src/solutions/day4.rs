use std::collections::HashSet;

struct Card {
    winning_nums: HashSet<u8>,
    given_nums: Vec<u8>,
    hits: u8,
    id: u32,
}

impl Card {
    fn parse(line: &str) -> Self {
        let mut winning_nums = HashSet::new();
        let mut given_nums = Vec::new();
        let mut hits = 0;
        let mut sections = line.split('|');
        let mut first_half = sections.next().unwrap().split(':');
        let id = first_half
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        first_half
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u8>().unwrap())
            .for_each(|num| {
                winning_nums.insert(num);
            });
        sections
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u8>().unwrap())
            .for_each(|num| {
                given_nums.push(num);
                if winning_nums.contains(&num) {
                    hits += 1;
                }
            });

        Self {
            winning_nums,
            given_nums,
            hits,
            id,
        }
    }

    fn score(&self) -> u32 {
        match self.hits {
            0 => 0,
            n => 1 << (n - 1),
        }
    }
}

pub fn run(input: String) -> Option<(String, String)> {
    let mut total_score = 0;
    input.lines().for_each(|line| {
        let game = Card::parse(line);
        total_score += game.score();
    });

    let cards = input.lines().map(Card::parse).collect::<Vec<Card>>();
    let mut counts = vec![1; cards.len()];

    for i in 0..cards.len() {
        let count = counts[i];
        let num_hits = cards[i].hits;
        for j in 0..num_hits as usize {
            counts[i + j + 1] += count;
        }
    }

    let part2 = counts.into_iter().sum::<u32>();
    Some((total_score.to_string(), part2.to_string()))
}
