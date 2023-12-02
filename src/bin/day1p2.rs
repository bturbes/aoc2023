use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day1.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let first_re = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let last_re = Regex::new(r".*(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let line_parser = LineParser { first_re, last_re };

    let result: i64 = lines
        .flatten()
        .map(|l| line_parser.parse_line(l.as_str()))
        .sum();

    println!("{}", result);
}

struct LineParser {
    first_re: Regex,
    last_re: Regex,
}

impl LineParser {
    fn parse_line(&self, line: &str) -> i64 {
        let first = self
            .first_re
            .captures(line)
            .map_or(None, |c| c.get(1))
            .map_or(0, |m| string_to_digit(m.as_str()));

        let last = self
            .last_re
            .captures(line)
            .map_or(None, |c| c.get(1))
            .map_or(first, |m| string_to_digit(m.as_str()));

        return format!("{}{}", first, last).parse::<i64>().unwrap();
    }
}

fn string_to_digit(s: &str) -> i64 {
    match s {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0,
    }
}
