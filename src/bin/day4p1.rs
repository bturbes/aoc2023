use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day4.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let result: u64 = lines
        .flatten()
        .map(|l| parse_card(l.as_str()))
        .map(|c| score_card(&c))
        .sum();

    println!("{}", result);
}

#[derive(Debug)]
struct Card {
    _id: u64,
    winning_numbers: Vec<u64>,
    numbers: Vec<u64>,
}

fn parse_card(line: &str) -> Card {
    let (card, numbers) = line.split_once(":").unwrap();
    let card_id = card
        .split_whitespace()
        .skip(1)
        .next()
        .and_then(|n| n.parse().ok())
        .unwrap();

    let (winning_str, number_str) = numbers.split_once("|").unwrap();
    let winning_str = winning_str.trim();
    let numbers_str = number_str.trim();

    let winning_numbers: Vec<u64> = winning_str
        .split_whitespace()
        .map(|n| n.parse::<u64>())
        .flatten()
        .collect();

    let numbers: Vec<u64> = numbers_str
        .split_whitespace()
        .map(|n| n.parse::<u64>())
        .flatten()
        .collect();

    let card = Card {
        _id: card_id,
        winning_numbers,
        numbers,
    };

    return card;
}

fn score_card(card: &Card) -> u64 {
    let num_winners = card
        .numbers
        .iter()
        .filter(|n| card.winning_numbers.contains(n))
        .count();

    if num_winners == 0 {
        return 0;
    }

    return (2 as u64).pow((num_winners - 1) as u32);
}
