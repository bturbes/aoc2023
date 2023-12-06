use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day4.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let cards: Vec<Card> = lines.flatten().map(|l| parse_card(l.as_str())).collect();
    let result = count_copies(&cards);

    println!("{}", result);
}

fn count_copies(cards: &Vec<Card>) -> usize {
    let mut counts: Vec<usize> = vec![1; cards.len() + 1];
    counts[0] = 0;

    for c in cards.iter() {
        let copies = counts[c.id];
        for count in counts[c.id + 1..=c.id + c.winners].iter_mut() {
            *count += copies;
        }
    }

    return counts.iter().sum();
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winners: usize,
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

    let winning_numbers: Vec<usize> = winning_str
        .split_whitespace()
        .map(|n| n.parse::<usize>())
        .flatten()
        .collect();

    let numbers: Vec<usize> = numbers_str
        .split_whitespace()
        .map(|n| n.parse::<usize>())
        .flatten()
        .collect();

    let winners = count_winners(&winning_numbers, &numbers);

    let card = Card {
        id: card_id,
        winners,
    };

    return card;
}

fn count_winners(winning_numbers: &Vec<usize>, numbers: &Vec<usize>) -> usize {
    return numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count();
}
