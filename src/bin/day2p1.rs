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
        .filter_map(|g| if is_possible(&g) { Some(g.id) } else { None })
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
    id: u64,
    handfuls: Vec<RGB>,
}

fn parse_game(line: &str) -> Game {
    let (game, handfuls) = line.split_once(":").unwrap();
    let game_id = game
        .split_once(" ")
        .and_then(|(_, s)| s.parse::<u64>().ok())
        .unwrap();

    let mut game = Game {
        id: game_id,
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

fn is_possible(game: &Game) -> bool {
    for hand in &game.handfuls {
        if hand.red > 12 {
            return false;
        }

        if hand.green > 13 {
            return false;
        }

        if hand.blue > 14 {
            return false;
        }
    }

    return true;
}
