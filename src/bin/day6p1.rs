use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day6.txt").unwrap();
    let mut lines = BufReader::new(file).lines().flatten();

    let times: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<usize> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let result: usize = times
        .iter()
        .zip(distances.iter())
        .map(|(time, dist)| (smallest_winner(*time, *dist), largest_winner(*time, *dist)))
        .map(|(min, max)| max.unwrap() - min.unwrap() + 1)
        .product();

    println!("{}", result);
}

fn smallest_winner(time: usize, dist: usize) -> Option<usize> {
    let mut l = 0;
    let mut r = time;
    let mut winner: Option<usize> = None;

    while l <= r {
        let mid = (l + r) / 2;
        let total_dist = calc_distance(time, mid);
        if total_dist > dist {
            winner = Some(mid);
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    return winner;
}

fn largest_winner(time: usize, dist: usize) -> Option<usize> {
    let mut l = 0;
    let mut r = time;
    let mut winner: Option<usize> = None;

    while l <= r {
        let mid = (l + r) / 2;
        let total_dist = calc_distance(time, mid);
        if total_dist > dist {
            winner = Some(mid);
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    return winner;
}
fn calc_distance(time: usize, hold: usize) -> usize {
    return (time - hold) * hold;
}
