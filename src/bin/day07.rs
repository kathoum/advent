use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut hands: Vec<(Hand, usize)> = BufReader::new(File::open("input/day07.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (hand, bid) = line.split_once(' ').unwrap();
            (Hand::from(hand), bid.parse().unwrap())
        })
        .collect();
    hands.sort();
    let answer = value(&hands);
    println!("Part One: {answer}");

    for hand in &mut hands {
        for c in &mut hand.0.cards {
            if *c == Card::J {
                *c = Card::Joker;
            }
        }
        hand.0.kind = Kind::from(&hand.0.cards);
    }

    hands.sort();
    let answer = value(&hands);
    println!("Part Two: {answer}");
}

fn value<'a>(hands: impl IntoIterator<Item = &'a (Hand, usize)>) -> usize {
    hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    kind: Kind,
    cards: [Card; 5],
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let cards = s
            .chars()
            .map(Card::from)
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();
        let kind = Kind::from(&cards);
        Hand { kind, cards }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    HighCard,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

impl From<&[Card; 5]> for Kind {
    fn from(cards: &[Card; 5]) -> Self {
        let mut counts = std::collections::HashMap::<Card, i32>::new();
        for c in cards {
            *counts.entry(*c).or_default() += 1;
        }

        let jokers = counts.get(&Card::Joker).copied().unwrap_or_default();
        counts.remove(&Card::Joker);
        let mut counts = counts.values().cloned().collect::<Vec<i32>>();
        counts.sort();
        match counts.last_mut() {
            Some(c) => *c += jokers,
            None => counts.push(jokers),
        };

        match counts[..] {
            [5] => Kind::Five,
            [1, 4] => Kind::Four,
            [2, 3] => Kind::Full,
            [1, 1, 3] => Kind::Three,
            [1, 2, 2] => Kind::TwoPair,
            [1, 1, 1, 2] => Kind::Pair,
            [1, 1, 1, 1, 1] => Kind::HighCard,
            _ => panic!("{:?}", counts),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Card::C2,
            '3' => Card::C3,
            '4' => Card::C4,
            '5' => Card::C5,
            '6' => Card::C6,
            '7' => Card::C7,
            '8' => Card::C8,
            '9' => Card::C9,
            'T' => Card::T,
            'J' => Card::J,
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => panic!("{}", c),
        }
    }
}
