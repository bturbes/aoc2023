use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day2.txt").unwrap();
    let lines = BufReader::new(file).lines();

    let result: u64 = lines
        .flatten()
        .map(|l| parse_game(l.as_str()))
        .map(|g| get_power(&g))
        .sum();

    println!("{}", result);
}

#[derive(Debug)]
struct RGB {
    red: u64,
    green: u64,
    blue: u64,
}

#[derive(Debug)]
struct Game {
    _id: u64,
    handfuls: Vec<RGB>,
}

fn parse_game(line: &str) -> Game {
    let (game, handfuls) = line.split_once(":").unwrap();
    let game_id = game
        .split_once(" ")
        .and_then(|(_, s)| s.parse::<u64>().ok())
        .unwrap();

    let mut game = Game {
        _id: game_id,
        handfuls: vec![],
    };

    let handfuls = handfuls.split(";");
    for hand in handfuls {
        let hand = hand.split(",");
        for color in hand {
            let (n, color) = color.trim().split_once(" ").unwrap();
            let n = n.parse::<u64>().unwrap();

            let mut rgb = RGB {
                red: 0,
                green: 0,
                blue: 0,
            };

            match color {
                "red" => rgb.red = n,
                "green" => rgb.green = n,
                "blue" => rgb.blue = n,
                _ => (),
            }

            game.handfuls.push(rgb);
        }
    }

    return game;
}

fn get_power(game: &Game) -> u64 {
    let mut max = RGB {
        red: 0,
        green: 0,
        blue: 0,
    };

    for hand in &game.handfuls {
        if hand.red > max.red {
            max.red = hand.red;
        }

        if hand.green > max.green {
            max.green = hand.green;
        }

        if hand.blue > max.blue {
            max.blue = hand.blue;
        }
    }

    return max.red * max.green * max.blue;
}
