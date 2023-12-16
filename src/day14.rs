use std::collections::{hash_map::DefaultHasher, HashMap, HashSet};
use std::hash::Hasher;

use glam::IVec2;
use itertools::Itertools;

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug)]
enum Rock {
    Movable,
    Stable,
}

fn count_rocks_that_would_fall_to_top(map: &str) -> u32 {
    // walk each column and count the rocks, O, until we hit a #, then move on.
    let col_count = map.lines().nth(0).unwrap().chars().count();
    let height = map.lines().count();

    (0..col_count)
        .map(|x| {
            let mut empty_space = 0;
            map.lines()
                .enumerate()
                .map(|(y, line)| match &line[x..=x] {
                    "O" => height - (y - empty_space),
                    "." => {
                        empty_space += 1;
                        0
                    }
                    "#" => {
                        empty_space = 0;
                        0
                    }
                    _ => unreachable!("inconceivable!"),
                })
                .sum::<usize>()
        })
        .sum::<usize>() as u32
}

pub fn day14_part1(input: &str) -> String {
    count_rocks_that_would_fall_to_top(input).to_string()
}

fn shift_rocks(max_position: IVec2, map: &mut HashMap<IVec2, Rock>, direction: Direction) {
    let direction_ivec = match direction {
        Direction::North => IVec2 { x: 0, y: -1 },
        Direction::West => IVec2 { x: -1, y: 0 },
        Direction::South => IVec2 { x: 0, y: 1 },
        Direction::East => IVec2 { x: 1, y: 0 },
    };

    for y in 0..=max_position.y {
        for x in 0..=max_position.x {
            // Should we be moving backwards?
            let (x, y) = match direction {
                Direction::South => (x, max_position.y - y),
                Direction::East => (max_position.x - x, y),
                _ => (x, y),
            };
            // does a movable rock live here?
            let mut position = IVec2 { x, y };
            if let Some(Rock::Movable) = map.get(&position) {
                // Now move this to the position it would lie in.
                loop {
                    let next_position = position + direction_ivec;
                    if next_position.x < 0
                        || next_position.x > max_position.x
                        || next_position.y < 0
                        || next_position.y > max_position.y
                    {
                        // off the map, break
                        break;
                    }
                    if let Some(_) = map.get(&next_position) {
                        break;
                    } else {
                        // this rock can move to this spot, take it.
                        map.remove(&position);
                        map.insert(next_position, Rock::Movable);
                        position = next_position;
                    }
                }
            }
        }
    }
}

// used when debugging.
fn rocks_as_str(max_position: IVec2, map: &HashMap<IVec2, Rock>) -> String {
    let mut result = "".to_string();
    for y in 0..=max_position.y {
        for x in 0..=max_position.x {
            result.push(match map.get(&IVec2 { x, y }) {
                Some(Rock::Movable) => 'O',
                Some(Rock::Stable) => '#',
                None => '.',
            });
        }
        result.push('\n');
    }
    result
}

pub fn day14_part2(input: &str) -> String {
    day14_part2_inner(input, 1_000_000_000)
}

fn rotate_rocks(max_position: IVec2, map: &mut HashMap<IVec2, Rock>) {
    shift_rocks(max_position, map, Direction::North);
    shift_rocks(max_position, map, Direction::West);
    shift_rocks(max_position, map, Direction::South);
    shift_rocks(max_position, map, Direction::East);
}

pub fn day14_part2_inner(input: &str, runs: u32) -> String {
    // parse the input in to a hashmap of position > rock type, ignore air.
    let mut map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    let x = x as i32;
                    let y = y as i32;
                    match c {
                        'O' => Some((IVec2 { x, y }, Rock::Movable)),
                        '#' => Some((IVec2 { x, y }, Rock::Stable)),
                        '.' => None,
                        _ => {
                            dbg!(c);
                            unreachable!()
                        }
                    }
                })
                .collect::<HashMap<IVec2, Rock>>()
        })
        .collect::<HashMap<IVec2, Rock>>();

    let max_position = IVec2 {
        x: input.lines().nth(0).unwrap().chars().count() as i32 - 1,
        y: input.lines().count() as i32 - 1,
    };

    // 1 cache to know when we should start the loop check cache.
    // when this cache stops growing we start putting elements in to the loop cache. When it stops
    // growing we know the # of elements in our loop.
    let mut cache: HashSet<u64> = HashSet::new();
    let mut loop_cache: HashSet<u64> = HashSet::new();

    for i in 0..runs {
        rotate_rocks(max_position, &mut map);

        // reduce the map state in to a unique key to put in a set to identify when we're looping.
        // use the hash of a string as an easy way to make sure its unique
        let v = (0..=max_position.y)
            .map(|y| {
                (0..=max_position.x).fold("".to_string(), |acc, x| {
                    if map.get(&IVec2 { x, y }).is_some() {
                        format!("{acc}x{x}x{y}").to_string()
                    } else {
                        acc
                    }
                })
            })
            .join(".");
        let mut hasher = DefaultHasher::new();
        hasher.write(v.as_bytes());
        let v = hasher.finish();

        if cache.get(&v).is_some() {
            if loop_cache.get(&v).is_some() {
                let remainder = ((runs - (i + 1)) as usize % loop_cache.len()) as i32;
                for _ in 0..remainder {
                    rotate_rocks(max_position, &mut map);
                }
                let score = map
                    .iter()
                    .filter_map(|(position, rock)| match rock {
                        Rock::Movable => Some(1 + max_position.y - position.y),
                        Rock::Stable => None,
                    })
                    .sum::<i32>();
                return score.to_string();
            } else {
                loop_cache.insert(v);
            }
        } else {
            // we just added a new entry to the cache, reset the loop cache
            loop_cache = HashSet::new();
        }
        cache.insert(v);
    }

    unreachable!("looped forever? unpossible");
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        "136"
    )]
    #[case(
        ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....",
        "129"
    )]
    fn test_day14_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(day14_part1(input), expected);
    }

    #[rstest]
    #[case(
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        1_000_000_000,
        "64"
    )]
    fn test_day14_part2(#[case] input: &str, #[case] runs: u32, #[case] expected: &str) {
        assert_eq!(day14_part2_inner(input, runs), expected);
    }
}
