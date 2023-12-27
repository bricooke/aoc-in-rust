use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;

fn next_steps(bounds: &IVec2, map: &HashMap<IVec2, char>, from: HashSet<IVec2>) -> HashSet<IVec2> {
    assert!(bounds.x == bounds.y);
    let positions_to_test = vec![IVec2::X, IVec2::NEG_X, IVec2::NEG_Y, IVec2::Y];

    from.iter()
        .flat_map(|from| {
            positions_to_test
                .iter()
                .map(|diff| *from + *diff)
                .filter_map(|point| {
                    match map.get(&IVec2::new(
                        point.x.rem_euclid(bounds.x),
                        point.y.rem_euclid(bounds.y),
                    )) {
                        Some(c) => match c {
                            '.' | 'S' => Some(point),
                            _ => None,
                        },
                        None => unreachable!(),
                    }
                })
                .collect_vec()
        })
        .collect::<HashSet<IVec2>>()
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
                (point, c)
            })
        })
        .collect::<HashMap<IVec2, char>>();

    let start = map
        .iter()
        .find(|(_k, v)| **v == 'S')
        .expect("should always have a start");

    let bounds_y = input.lines().count();
    let bounds_x = input.lines().nth(0).unwrap().chars().count();
    let bounds = IVec2::new(bounds_x as i32, bounds_y as i32);

    let mut next = HashSet::new();
    next.insert(*start.0);
    for _ in 1..=steps_left {
        next = next_steps(&bounds, &map, next);
    }

    next.len().to_string()
}

pub fn day21_part2(input: &str) -> String {
    day21_part2_inner(input, 26_501_365)
}

fn day21_part2_inner(input: &str, steps_left: usize) -> String {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let point = IVec2 {
                    x: x as i32,
                    y: y as i32,
                };
                (point, c)
            })
        })
        .collect::<HashMap<IVec2, char>>();

    let start = map
        .iter()
        .find(|(_k, v)| **v == 'S')
        .expect("should always have a start");

    // credit: https://nickymeuleman.netlify.app/garden/aoc2023-day21
    let bounds_y = input.lines().count();
    let bounds_x = input.lines().nth(0).unwrap().chars().count();
    let size = bounds_x as u64;
    let bounds = IVec2::new(bounds_x as i32, bounds_y as i32);
    let to_edge = (bounds.x / 2) as u64;
    let mut factors = Vec::new();
    let goal = steps_left as u64;

    let mut next = HashSet::new();
    next.insert(*start.0);
    for count in 1..=steps_left {
        next = next_steps(&bounds, &map, next);

        if count as u64 == to_edge + (size * (factors.len() as u64)) {
            factors.push(next.len() as u64);

            if factors.len() == 3 {
                let delta0 = factors[0];
                let delta1 = factors[1] - factors[0];
                let delta2 = factors[2] - 2 * factors[1] + factors[0];

                return (delta0
                    + delta1 * (goal / size)
                    + delta2 * ((goal / size) * ((goal / size) - 1) / 2))
                    .to_string();
            }
        }
    }
    next.len().to_string()
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
    fn test_day21_part2() {
        assert_eq!("16", day21_part2_inner(INPUT, 6));
        assert_eq!("50", day21_part2_inner(INPUT, 10));
    }
}
