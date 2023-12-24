use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;

fn next_steps(map: &HashMap<IVec2, char>, from: HashSet<IVec2>) -> HashSet<IVec2> {
    let positions_to_test = vec![IVec2::X, IVec2::NEG_X, IVec2::NEG_Y, IVec2::Y];

    // TODO: don't go back to places we've been.

    let next = from
        .iter()
        .flat_map(|from| {
            positions_to_test
                .iter()
                .filter_map(|diff| match map.get(&(*from + *diff)) {
                    Some(c) => match c {
                        '.' => Some(*from + *diff),
                        _ => None,
                    },
                    None => None,
                })
                .collect_vec()
        })
        .collect::<HashSet<IVec2>>();

    next
}

pub fn day21_part1(input: &str) -> String {
    day21_part1_inner(input, 64)
}

fn day21_part1_inner(input: &str, steps_left: usize) -> String {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let point = IVec2 {
                    x: x as i32,
                    y: y as i32,
                };
                if c == 'S' {
                    dbg!(point);
                }
                (point, c)
            })
        })
        .collect::<HashMap<IVec2, char>>();

    let start = map
        .iter()
        .find(|(_k, v)| **v == 'S')
        .expect("should always have a start");

    let mut next = HashSet::new();
    next.insert(*start.0);
    for _ in 1..=steps_left {
        next = next_steps(&map, next);
    }

    (next.len() + 1).to_string()
}

pub fn day21_part2(_input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_day21_part1() {
        assert_eq!("16", day21_part1_inner(INPUT, 6));
    }

    #[test]
    #[ignore]
    fn test_day21_part2() {
        assert_eq!("0", day21_part2(INPUT));
    }
}
