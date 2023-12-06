use crate::day5::parser::parse;
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct Range {
    destination: u64,
    source: u64,
    length: u64,
}

#[derive(Debug)]
struct Map {
    _title: String,
    ranges: Vec<Range>,
}

impl Map {
    fn to_destination(&self, from: u64) -> u64 {
        for range in &self.ranges {
            if range.source <= from && from < range.source + range.length {
                let diff = from - range.source;
                return range.destination + diff;
            }
        }
        from
    }
}

mod parser {
    use super::*;

    pub fn parse(input: &str) -> Vec<Map> {
        let mut lines = input.lines();
        let _ = lines.next();
        let mut title: &str = "";
        let mut map_lines: Vec<&str> = Vec::new();
        let mut maps: Vec<Map> = Vec::new();

        fn create_map(map_lines: &Vec<&str>, title: String) -> Map {
            let ranges = map_lines
                .iter()
                .map(|range| {
                    let range = range
                        .split(" ")
                        .map(|r| r.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    Range {
                        destination: range[0],
                        source: range[1],
                        length: range[2],
                    }
                })
                .collect::<Vec<_>>();

            Map {
                _title: title,
                ranges,
            }
        }

        for line in lines {
            if line.len() == 0 {
                continue;
            }
            if line
                .chars()
                .collect::<Vec<_>>()
                .first()
                .unwrap()
                .is_alphabetic()
            {
                if title.len() > 0 {
                    maps.push(create_map(&map_lines, title.to_string()));
                    map_lines = vec![];
                }
                title = line;
            } else {
                map_lines.push(line);
            }
        }
        maps.push(create_map(&map_lines, title.to_string()));
        maps
    }
}

pub fn day5_part1(input: &str) -> String {
    let mut lines = input.lines();
    let seed_line = lines.next();

    let seeds = seed_line.unwrap().split(": ").skip(1).next().unwrap();
    let seeds = seeds.split(" ").collect::<Vec<_>>();
    let seeds = seeds.iter().map(|d| d.parse::<u64>().unwrap());

    let maps = parse(input);

    seeds
        .map(|seed| maps.iter().fold(seed, |next, map| map.to_destination(next)))
        .min()
        .unwrap()
        .to_string()
}

pub fn day5_part2(input: &str) -> String {
    let mut lines = input.lines();
    let seed_line = lines.next();

    let seeds = seed_line.unwrap().split(": ").skip(1).next().unwrap();
    let seeds = seeds.split(" ").collect::<Vec<_>>();
    let seeds = seeds.iter().map(|d| d.parse::<u64>().unwrap());

    let maps = parse(input);

    // Brute force! This takes minutes to run, but gets the correct answer.
    seeds
        .chunks(2)
        .into_iter()
        .map(|seed_chunk| {
            let chunks = seed_chunk.collect::<Vec<u64>>();
            assert_eq!(2, chunks.len());
            (chunks[0]..chunks[0] + chunks[1])
                .map(|seed| {
                    maps.iter()
                        .fold(seed, |next, map| map.to_destination(next).clone())
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_day5_part1() {
        assert_eq!("35", day5_part1(INPUT));
    }

    #[test]
    fn test_day5_part2() {
        assert_eq!("46", day5_part2(INPUT));
    }
}
