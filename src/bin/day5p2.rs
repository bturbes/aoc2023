use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

fn main() {
    let file = File::open("inputs/day5.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();
    let almanac = parse_almanac(lines);

    let result = almanac
        .seeds
        .iter()
        .flat_map(|s| s.into_iter())
        .map(|s| almanac.map(s))
        .min()
        .unwrap();

    println!("{}", result);
}

fn parse_almanac(lines: Vec<String>) -> Almanac {
    let seeds: Vec<SeedRange> = lines
        .iter()
        .next()
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
        .map(|pairs| SeedRange {
            start: pairs[0],
            range: pairs[1],
        })
        .collect();

    let seed_to_soil: SeedMap = SeedMap::new(
        lines
            .iter()
            .skip_while(|l| !l.contains("seed-to-soil"))
            .skip(1)
            .take_while(|l| *l != "")
            .map(|l| SeedMapEntry::from_str(l.as_str()))
            .flatten()
            .collect(),
    );

    let soil_to_fertilizer: SeedMap = SeedMap::new(
        lines
            .iter()
            .skip_while(|l| !l.contains("soil-to-fertilizer"))
            .skip(1)
            .take_while(|l| *l != "")
            .map(|l| SeedMapEntry::from_str(l.as_str()))
            .flatten()
            .collect(),
    );

    let fertilizer_to_water: SeedMap = SeedMap::new(
        lines
            .iter()
            .skip_while(|l| !l.contains("fertilizer-to-water"))
            .skip(1)
            .take_while(|l| *l != "")
            .map(|l| SeedMapEntry::from_str(l.as_str()))
            .flatten()
            .collect(),
    );

    let water_to_light: SeedMap = SeedMap::new(
        lines
            .iter()
            .skip_while(|l| !l.contains("water-to-light"))
            .skip(1)
            .take_while(|l| *l != "")
            .map(|l| SeedMapEntry::from_str(l.as_str()))
            .flatten()
            .collect(),
    );

    let light_to_temperature: SeedMap = SeedMap::new(
        lines
            .iter()
            .skip_while(|l| !l.contains("light-to-temperature"))
            .skip(1)
            .take_while(|l| *l != "")
            .map(|l| SeedMapEntry::from_str(l.as_str()))
            .flatten()
            .collect(),
    );

    let temperature_to_humidity: SeedMap = SeedMap::new(
        lines
            .iter()
            .skip_while(|l| !l.contains("temperature-to-humidity"))
            .skip(1)
            .take_while(|l| *l != "")
            .map(|l| SeedMapEntry::from_str(l.as_str()))
            .flatten()
            .collect(),
    );

    let humidity_to_location: SeedMap = SeedMap::new(
        lines
            .iter()
            .skip_while(|l| !l.contains("humidity-to-location"))
            .skip(1)
            .take_while(|l| *l != "")
            .map(|l| SeedMapEntry::from_str(l.as_str()))
            .flatten()
            .collect(),
    );

    return Almanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    };
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<SeedRange>,
    seed_to_soil: SeedMap,
    soil_to_fertilizer: SeedMap,
    fertilizer_to_water: SeedMap,
    water_to_light: SeedMap,
    light_to_temperature: SeedMap,
    temperature_to_humidity: SeedMap,
    humidity_to_location: SeedMap,
}

impl Almanac {
    fn map(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);
        let location = self.humidity_to_location.map(humidity);

        return location;
    }
}

#[derive(Debug, Default)]
struct SeedRange {
    start: usize,
    range: usize,
}

impl<'a> IntoIterator for &'a SeedRange {
    type Item = usize;

    type IntoIter = SeedRangeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            seed_range: self,
            current: self.start,
        }
    }
}

struct SeedRangeIter<'a> {
    seed_range: &'a SeedRange,
    current: usize,
}

impl<'a> Iterator for SeedRangeIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.current;
        self.current += 1;
        if n < self.seed_range.start + self.seed_range.range {
            return Some(n);
        }

        return None;
    }
}

#[derive(Debug, Default)]
struct SeedMap {
    entries: Vec<SeedMapEntry>,
}

impl SeedMap {
    fn new(entries: Vec<SeedMapEntry>) -> Self {
        Self { entries }
    }

    fn map(&self, source: usize) -> usize {
        for entry in &self.entries {
            if source >= entry.source && source < entry.source + entry.range {
                return entry.destination + source - entry.source;
            }
        }

        return source;
    }
}

#[derive(Debug, Default)]
struct SeedMapEntry {
    source: usize,
    destination: usize,
    range: usize,
}

impl FromStr for SeedMapEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mappings = s.split(" ").map(|s| s.parse::<usize>()).flatten();

        return Ok(SeedMapEntry {
            destination: mappings.next().ok_or(())?,
            source: mappings.next().ok_or(())?,
            range: mappings.next().ok_or(())?,
        });
    }
}
