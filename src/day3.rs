use std::collections::HashMap;

use itertools::Itertools;
use Item::*;

#[derive(Debug)]
struct NumberDetails {
    amount: i32,
    position: (usize, usize),
    length: usize,
}

#[derive(Debug)]
enum Item {
    Number(NumberDetails),
    Symbol((usize, usize), char),
}

impl Item {
    fn position(&self) -> (usize, usize) {
        match self {
            Number(details) => details.position,
            Symbol(position, _) => *position,
        }
    }
}

fn parse(input: &str) -> Vec<Item> {
    let mut items: Vec<Item> = Vec::new();
    let mut current = "".to_string();
    input.lines().enumerate().for_each(|(y, line)| {
        current = "".to_string();
        line.chars().enumerate().for_each(|(x, c)| match c {
            '0'..='9' => {
                current.push_str(&c.to_string());

                // yuck, this is done 3 times :(
                if x + 1 == line.len() && current.len() > 0 {
                    let amount = current.parse::<i32>().unwrap();
                    items.push(Number(NumberDetails {
                        amount,
                        position: (x + 1 - current.len(), y),
                        length: current.len(),
                    }));
                }
            }
            '.' => {
                if current.len() > 0 {
                    let amount = current.parse::<i32>().unwrap();
                    items.push(Number(NumberDetails {
                        amount,
                        position: (x - current.len(), y),
                        length: current.len(),
                    }));
                }
                current = "".to_string();
            }
            c => {
                if current.len() > 0 {
                    let amount = current.parse::<i32>().unwrap();
                    items.push(Number(NumberDetails {
                        amount,
                        position: (x - current.len(), y),
                        length: current.len(),
                    }));
                }
                items.push(Symbol((x, y), c));
                current = "".to_string();
            }
        });
    });

    items
}

impl Item {
    fn is_number_and_touching(&self, map: &HashMap<(usize, usize), &Item>) -> Option<i32> {
        match self {
            Number(details) => {
                // look at each occupied position and check its surroundings
                // part1 end note: Lost so much time due to having 0..=details.position.0...
                // instead of .0..details... :(
                // so many of the tests didn't catch that and it didn't reproduce in small snippets. :/
                for x in details.position.0..details.position.0 + details.length {
                    let (x, y) = (x as i32, details.position.1 as i32);
                    // Now look all around
                    let x_range = x - 1..=x + 1;
                    let y_range = y - 1..=y + 1;
                    let positions = x_range
                        .into_iter()
                        .cartesian_product(y_range.into_iter())
                        .collect::<Vec<_>>();

                    let any = positions.iter().any(|position| {
                        let item = map.get(&(position.0 as usize, position.1 as usize));
                        match item {
                            Some(Symbol(_, ..)) => true,
                            _ => false,
                        }
                    });

                    if any {
                        return Some(details.amount);
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn gear_ratio(&self, map: &HashMap<(usize, usize), &Item>) -> Option<i32> {
        match self {
            Symbol((x, y), '*') => {
                // Now look all around
                let x_range = x - 1..=x + 1;
                let y_range = y - 1..=y + 1;
                let positions = x_range
                    .into_iter()
                    .cartesian_product(y_range.into_iter())
                    .collect::<Vec<_>>();

                let numbers = positions
                    .iter()
                    .filter_map(|position| {
                        let item = map.get(&(position.0 as usize, position.1 as usize));
                        match item {
                            Some(Number(details)) => Some(details),
                            _ => None,
                        }
                    })
                    .unique_by(|details| details.position)
                    .map(|details| details.amount)
                    .collect::<Vec<_>>();

                if numbers.len() == 2 {
                    Some(numbers.iter().product::<i32>())
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

pub fn day3_part1(input: &str) -> String {
    let input = input.trim();
    let items = parse(input);
    let mut map: HashMap<(usize, usize), &Item> = HashMap::new();

    for item in items.as_slice() {
        map.insert(item.position(), item);
    }

    items
        .iter()
        .filter_map(|item| item.is_number_and_touching(&map))
        .sum::<i32>()
        .to_string()
}

pub fn day3_part2(input: &str) -> String {
    let input = input.trim();
    let items = parse(input);
    let mut map: HashMap<(usize, usize), &Item> = HashMap::new();

    for item in items.as_slice() {
        match item {
            Number(details) => {
                (details.position.0..details.position.0 + details.length).for_each(|x| {
                    map.insert((x, details.position.1), item);
                });
            }
            _ => (),
        }
    }

    let gears = items
        .iter()
        .filter(|item| match item {
            Symbol(_, c) => return *c == '*',
            _ => false,
        })
        .collect::<Vec<_>>();

    gears
        .iter()
        .filter_map(|gear| gear.gear_ratio(&map))
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.";
    #[test]
    fn test_day3_part1() {
        assert_eq!("4361", day3_part1(INPUT));
    }

    #[test]
    #[ignore]
    fn test_day3_part1_matching_last() {
        assert_eq!(
            "3",
            day3_part1(
                "
1.....
.-...1
......
...1-5
.....6"
            )
        );
    }

    #[test]
    fn test_day3_part1_reddit_help2() {
        assert_eq!(
            "156",
            day3_part1(
                "
....................
..@52..52-..52..52..
..................-.
"
            )
        );
    }

    #[test]
    fn test_day3_part1_reddit_help() {
        assert_eq!(
            "508",
            day3_part1(
                "
324....508
....508.
....../..."
            )
        );
    }

    #[test]
    fn test_day3_part1_bisected() {
        assert_eq!(
            "1020",
            day3_part1(
                "
..592....#...64..*..........*........
...*.........*..........839.364......
                "
            )
        );
    }

    #[test]
    fn test_day3_part2() {
        assert_eq!("467835", day3_part2(INPUT));
    }
}
