use std::{
    borrow::BorrowMut,
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("inputs/day3.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();

    let mut parts: HashMap<(usize, usize), u64> = HashMap::new();
    let mut gear_parts_locations: HashMap<(usize, usize), HashSet<(usize, usize)>> = HashMap::new();
    let mut local_gears: HashSet<(usize, usize)> = HashSet::new();
    let mut part_location: Option<(usize, usize)> = None;
    let mut part = "".to_string();
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if part_location.is_none() {
                    part_location = Some((i, j));
                }
                part.push(c);
                find_adjacent_gears(i, j, &lines, &mut local_gears);
            } else {
                if let Some(part_location) = part_location {
                    parts.insert(part_location, part.parse().unwrap());
                    assign_gear_parts(part_location, &local_gears, &mut gear_parts_locations)
                }
                part_location = None;
                part = "".to_string();
                local_gears.clear();
            }
        }
        if let Some(part_location) = part_location {
            parts.insert(part_location, part.parse().unwrap());
            assign_gear_parts(part_location, &local_gears, &mut gear_parts_locations)
        }
        part_location = None;
        part = "".to_string();
        local_gears.clear();
    }

    let result: u64 = gear_parts_locations
        .into_values()
        .map(|part_locations| gear_ratio(&part_locations, &parts))
        .flatten()
        .sum();

    println!("{}", result);
}

fn find_adjacent_gears(
    i: usize,
    j: usize,
    lines: &Vec<String>,
    gears: &mut HashSet<(usize, usize)>,
) {
    let i_start = max(0, i as i64 - 1) as usize;
    let i_end = min((lines.len() - 1) as i64, i as i64 + 1) as usize;

    let j_start = max(0, j as i64 - 1) as usize;
    let j_end = min((lines.len() - 1) as i64, j as i64 + 1) as usize;

    for i in i_start..=i_end {
        let chars: Vec<char> = lines[i].chars().collect();
        for j in j_start..=j_end {
            if chars[j] == '*' {
                gears.insert((i, j));
            }
        }
    }
}

fn assign_gear_parts(
    part_location: (usize, usize),
    local_gears: &HashSet<(usize, usize)>,
    gear_parts: &mut HashMap<(usize, usize), HashSet<(usize, usize)>>,
) {
    for gear in local_gears.iter() {
        let mut parts = gear_parts.get_mut(gear);
        if let Some(parts) = parts.borrow_mut() {
            parts.insert(part_location);
        } else {
            let mut parts = HashSet::new();
            parts.insert(part_location);
            gear_parts.insert(*gear, parts);
        }
    }
}

fn gear_ratio(
    part_locations: &HashSet<(usize, usize)>,
    parts: &HashMap<(usize, usize), u64>,
) -> Option<u64> {
    if part_locations.len() == 2 {
        let mut gears_iter = part_locations.iter();
        let first_location = gears_iter.next().unwrap();
        let second_location = gears_iter.next().unwrap();
        let first = parts.get(first_location).unwrap();
        let second = parts.get(second_location).unwrap();

        return Some(first * second);
    }

    return None;
}
