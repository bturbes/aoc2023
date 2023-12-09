use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day8.txt").unwrap();
    let mut lines = BufReader::new(file).lines().flatten();

    let path: Vec<Direction> = lines.next().unwrap().chars().map(|c| c.into()).collect();

    let mut index_map: HashMap<String, usize> = HashMap::new();
    let mappings: Vec<(String, String, String)> = lines
        .skip(1)
        .map(|l| {
            (
                l[0..3].to_string(),
                l[7..10].to_string(),
                l[12..15].to_string(),
            )
        })
        .collect();

    mappings
        .iter()
        .map(|(n, _, _)| n)
        .enumerate()
        .for_each(|(i, n)| {
            index_map.insert(n.clone(), i);
        });

    let mut nodes: Vec<Node> = Vec::with_capacity(mappings.len());
    mappings.iter().for_each(|(n, l, r)| {
        nodes.push(Node {
            id: n.clone(),
            l_idx: *index_map.get(l).unwrap(),
            r_idx: *index_map.get(r).unwrap(),
        })
    });

    let results: Vec<usize> = nodes
        .iter()
        .enumerate()
        .filter(|(_, n)| n.id.as_bytes()[2] as char == 'A')
        .map(|(i, _)| {
            let mut count = 0;
            let mut idx = i;
            let mut path = path.iter().cycle();
            loop {
                let n = &nodes[idx];
                if n.id.as_bytes()[2] as char == 'Z' {
                    break;
                }

                count += 1;
                match path.next().unwrap() {
                    Direction::Left => idx = n.l_idx,
                    Direction::Right => idx = n.r_idx,
                }
            }
            return count;
        }).collect();

    println!(r#"go to wolfram alpha and run "least common multiple {:?}" lol!"#, results);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        if value == 'L' {
            Direction::Left
        } else {
            Direction::Right
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    id: String,
    l_idx: usize,
    r_idx: usize,
}
