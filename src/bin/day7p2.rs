use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    let file = File::open("inputs/day7.txt").unwrap();
    let lines = BufReader::new(file).lines().flatten();

    assert!(Card::Number(10) > Card::Number(2));
    assert!(Card::Ace > Card::Queen);

    let mut bids: Vec<Bid> = lines.map(|l| Bid::from_str(l.as_str())).flatten().collect();
    bids.sort();

    let result: usize = bids
        .iter()
        .enumerate()
        .map(|(n, bid)| bid.bid * (n + 1))
        .sum();

    println!("{}", result);
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Card {
    Joker,
    Number(u8),
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Joker,
            'T' => Card::Number(10),
            c => Card::Number(c.to_digit(10).unwrap() as u8),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Hand {
    HighCard([Card; 5]),
    OnePair([Card; 5]),
    TwoPair([Card; 5]),
    ThreeKind([Card; 5]),
    FullHouse([Card; 5]),
    FourKind([Card; 5]),
    FiveKind([Card; 5]),
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 5] = s
            .chars()
            .map(|c| c.into())
            .collect::<Vec<Card>>()
            .try_into()
            .unwrap();

        let mut counts: HashMap<Card, usize> = HashMap::new();
        for card in &cards {
            *counts.entry(*card).or_insert(0) += 1;
        }
        let jokers = *counts.get(&Card::Joker).unwrap_or(&0);
        counts.remove(&Card::Joker);

        let mut sorted: Vec<usize> = counts.values().map(|v| *v).collect();
        sorted.sort();
        sorted.reverse();

        // This is the edge case of all jokers
        if sorted.len() == 0 {
            return Ok(Hand::FiveKind(cards));
        }

        if sorted[0] == 5 || sorted[0] + jokers == 5 {
            return Ok(Hand::FiveKind(cards));
        }

        if sorted[0] == 4 || sorted[0] + jokers == 4 {
            return Ok(Hand::FourKind(cards));
        }

        if ((sorted[0] == 3 || sorted[0] + jokers == 3) && sorted[1] == 2)
            || (sorted[0] == 3 && (sorted[1] == 2 || sorted[1] + jokers == 2))
        {
            return Ok(Hand::FullHouse(cards));
        }

        if sorted[0] == 3 || sorted[0] + jokers == 3 {
            return Ok(Hand::ThreeKind(cards));
        }

        // A two pair will never have a joker
        if sorted[0] == 2 && sorted[1] == 2 {
            return Ok(Hand::TwoPair(cards));
        }

        if sorted[0] == 2 || sorted[0] + jokers == 2 {
            return Ok(Hand::OnePair(cards));
        }

        return Ok(Hand::HighCard(cards));
    }
}

#[derive(Debug, Eq, Ord)]
struct Bid {
    hand: Hand,
    bid: usize,
}

impl FromStr for Bid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(" ").unwrap();
        let bid = bid.parse().unwrap();
        let hand = Hand::from_str(cards).unwrap();

        Ok(Bid { bid, hand })
    }
}

impl PartialEq for Bid {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl PartialOrd for Bid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}
