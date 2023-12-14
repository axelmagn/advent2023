use std::{collections::HashSet, io::stdin, iter::repeat};

use clap::Parser;
use once_cell::sync::Lazy;
use regex::Regex;

static NUM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

pub fn main() {
    let args = Args::parse();
    match args.part {
        1 => solve_part1(),
        2 => solve_part2(),
        _ => panic!("Invalid part argument"),
    }
}

fn solve_part1() {
    let score = stdin()
        .lines()
        .map(|line| Card::parse(&line.unwrap()).unwrap().score())
        .fold(0, |acc, score| acc + score);
    println!("{}", score)
}

fn solve_part2() {
    let cards: Vec<Card> = stdin()
        .lines()
        .map(|line| Card::parse(&line.unwrap()).unwrap())
        .collect();
    let mut card_counts: Vec<u32> = repeat(1).take(cards.len()).collect();
    let mut total = 0;
    for (i, card) in cards.iter().enumerate() {
        let card_count = card_counts[i];
        total += card_count;
        let score = card.score_b() as usize;
        let start = (i + 1).min(cards.len());
        let end = (i + score + 1).min(cards.len());
        for j in start..end {
            card_counts[j] += card_count;
        }
    }
    println!("{}", total);
}

#[derive(Debug, Clone)]
enum Error {}

type Result<T> = std::result::Result<T, Error>;

struct Card {
    winners: HashSet<u32>,
    picks: Vec<u32>,
}

impl Card {
    fn new(winners: HashSet<u32>, picks: Vec<u32>) -> Self {
        Self { winners, picks }
    }

    fn parse(s: &str) -> Result<Self> {
        let mut split = s.trim().split(":");
        let _card_header = split.next().unwrap();
        let card_tail = split.next().unwrap();

        let mut split = card_tail.trim().split("|");
        let winners_str = split.next().unwrap();
        let winners = NUM_RE
            .find_iter(winners_str)
            .map(|m| m.as_str().parse().unwrap())
            .collect();
        let picks_str = split.next().unwrap();
        let picks = NUM_RE
            .find_iter(picks_str)
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        Ok(Self::new(winners, picks))
    }

    fn score(&self) -> u32 {
        let mut out = 0;
        for pick in &self.picks {
            if self.winners.contains(pick) {
                if out == 0 {
                    out = 1;
                } else {
                    out *= 2;
                }
            }
        }
        out
    }

    fn score_b(&self) -> u32 {
        self.picks
            .iter()
            .filter(|pick| self.winners.contains(pick))
            .count() as u32
    }
}

#[test]
fn test_card_parse() {
    let card = Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();
    assert!(card.winners.contains(&41));
    assert!(card.picks.contains(&17));
}

#[test]
fn test_card_score() {
    let card = Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();
    assert_eq!(card.score(), 8);
}

#[test]
fn test_card_score_b() {
    let card = Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();
    assert_eq!(card.score_b(), 4);
    let card = Card::parse("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").unwrap();
    assert_eq!(card.score_b(), 2);
}
