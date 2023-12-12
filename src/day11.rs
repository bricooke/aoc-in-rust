use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use valence_math::IVec2;

#[derive(Debug)]
enum Item {
    Empty,
    Galaxy(usize), // id
}
// ^ isn't really needed. There's a good bit of simplification that can be done in the solution
// here.

// NOT used in part2
// amount is the multiplier. so in part 1 its 2 for 2x each row.
// this is only used in part1. There's no need for it still to exist
// but I might come back and look at these some day (unlikely) and
// seems worth keeping how I got here mapped out.
fn expand_universe(input: &str, amount: u32) -> String {
    // expand rows
    let mut cols_with_galaxies: HashSet<usize> = HashSet::new();

    let universe = input
        .lines()
        .flat_map(|line| {
            if !line.contains("#") {
                (0..amount).map(|_| line).collect::<Vec<_>>()
            } else {
                line.chars().enumerate().for_each(|(x, c)| {
                    if c == '#' {
                        cols_with_galaxies.insert(x);
                    }
                });
                vec![line]
            }
        })
        .join("\n");

    let universe = universe
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if cols_with_galaxies.get(&x).is_none() {
                        (0..amount).map(|_| c.to_string()).join("")
                    } else {
                        c.to_string()
                    }
                })
                .join("")
        })
        .join("\n");
    universe
}

fn parse(input: &String) -> HashMap<IVec2, Item> {
    let mut galaxy_id = 0;
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, p)| match p {
                '.' => (
                    IVec2 {
                        x: x as i32,
                        y: y as i32,
                    },
                    Item::Empty,
                ),
                '#' => {
                    galaxy_id += 1;
                    (
                        IVec2 {
                            x: x as i32,
                            y: y as i32,
                        },
                        Item::Galaxy(galaxy_id.clone()),
                    )
                }
                _ => unreachable!(),
            })
        })
        .collect::<HashMap<IVec2, Item>>()
}

pub fn find_shortest_path(
    a: &IVec2,
    b: &IVec2,
    empty_cols: &Vec<usize>,
    empty_rows: &Vec<usize>,
    expansion: u32,
) -> u64 {
    if a == b {
        return 0;
    }

    let mut moves: u64 = 0;
    let next_x: i32;
    let next_y: i32;

    match a.x.cmp(&b.x) {
        std::cmp::Ordering::Less => next_x = a.x + 1,
        std::cmp::Ordering::Equal => next_x = a.x,
        std::cmp::Ordering::Greater => next_x = a.x - 1,
    }
    match a.y.cmp(&b.y) {
        std::cmp::Ordering::Less => next_y = a.y + 1,
        std::cmp::Ordering::Equal => next_y = a.y,
        std::cmp::Ordering::Greater => next_y = a.y - 1,
    }

    // Calculate the moves needed, and account for expansion if needed
    if next_y != a.y {
        moves = 1;
        if empty_rows.contains(&(next_y as usize)) {
            moves += (expansion as u64) - 1;
        }
    }
    if next_x != a.x {
        moves += 1;
        if empty_cols.contains(&(next_x as usize)) {
            moves += (expansion as u64) - 1;
        }
    }

    return moves
        + find_shortest_path(
            &IVec2 {
                x: next_x,
                y: next_y,
            },
            b,
            empty_cols,
            empty_rows,
            expansion,
        );
}

pub fn day11_part1(input: &str) -> String {
    // expand the universe first
    let universe = expand_universe(input, 2);
    // parse id and position of each galaxy
    let universe = parse(&universe);
    // generate pairs of galaxies
    let galaxies = universe
        .iter()
        .filter(|(_, item)| match item {
            Item::Galaxy(_) => true,
            _ => false,
        })
        .collect::<HashMap<&IVec2, &Item>>();
    // pair up all galaxies
    let galaxy_pairs = galaxies
        .keys()
        .flat_map(|k| {
            galaxies
                .keys()
                .flat_map(move |k2| if k2 == k { None } else { Some((k, k2)) })
        })
        .collect::<Vec<_>>();
    // find shortest path between each pair
    (galaxy_pairs
        .iter()
        .map(|(a, b)| find_shortest_path(a, b, &vec![], &vec![], 2))
        .sum::<u64>()
        / 2) // lazy...we calculated (0,0) > (1,1) as well as (1,1)>(0,0) so half it :facepalm:
    .to_string()
}

pub fn day11_part2(input: &str) -> String {
    day11_part2_inner(input, 1_000_000)
}

pub fn day11_part2_inner(input: &str, expansion_amount: u32) -> String {
    // uhhhh, don't expand anything. We're going to need to use some simple math.
    // parse id and position of each galaxy
    let universe = parse(&input.to_string());
    // generate ids of rows that are empty and cols that are empty
    let mut cols_with_galaxies: HashSet<usize> = HashSet::new();

    let empty_rows = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            if !line.contains("#") {
                Some(y)
            } else {
                line.chars().enumerate().for_each(|(x, c)| {
                    if c == '#' {
                        cols_with_galaxies.insert(x);
                    }
                });
                None // not empty
            }
        })
        .collect::<Vec<usize>>();

    let line = input.lines().next().unwrap();
    let empty_cols = line
        .chars()
        .enumerate()
        .filter_map(|(x, _)| {
            if cols_with_galaxies.contains(&x) {
                None
            } else {
                Some(x)
            }
        })
        .collect::<Vec<usize>>();

    // generate pairs of galaxies
    let galaxies = universe
        .iter()
        .filter(|(_, item)| match item {
            Item::Galaxy(_) => true,
            _ => false,
        })
        .collect::<HashMap<&IVec2, &Item>>();
    // pair up all galaxies
    let galaxy_pairs = galaxies
        .keys()
        .flat_map(|k| {
            galaxies
                .keys()
                .flat_map(move |k2| if k2 == k { None } else { Some((k, k2)) })
        })
        .collect::<Vec<_>>();
    // find shortest path between each pair
    (galaxy_pairs
        .iter()
        .map(|(a, b)| find_shortest_path(a, b, &empty_cols, &empty_rows, expansion_amount))
        .sum::<u64>()
        / 2) // lazy...we calculated (0,0) > (1,1) as well as (1,1)>(0,0) so half it :facepalm:
    .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;
    use valence_math::*;

    #[rstest]
    #[case(
        IVec2{x:0, y:0},
        IVec2{x:1,y:1},
        2
    )]
    #[case(
        IVec2{x:5, y:0},
        IVec2{x:1,y:0},
        4
    )]
    #[case(
        IVec2{x:5, y:11},
        IVec2{x:1,y:6},
        9
    )]
    fn test_day11_distances(#[case] a: IVec2, #[case] b: IVec2, #[case] expected: u64) {
        assert_eq!(expected, find_shortest_path(&a, &b, &vec![], &vec![], 2));
    }

    #[rstest]
    #[case(
        "...
..#
.#.",
        2,
        4,
        4
    )]
    #[case(
        "..
..", 10, 20, 20
    )]
    fn test_day11_expand(
        #[case] input: &str,
        #[case] expansion_amount: u32,
        #[case] row_count: u32,
        #[case] col_count: u32,
    ) {
        let expanded = expand_universe(input, expansion_amount);
        assert_eq!(row_count, expanded.lines().count() as u32);
        assert_eq!(
            col_count,
            expanded.lines().next().unwrap().chars().count() as u32
        );
    }

    #[rstest]
    #[case(
        "..#
..#", "1"
    )]
    #[case(
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        "374"
    )]
    fn test_day11_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, day11_part1(input));
    }

    #[rstest]
    #[case(
        "..#
...
..#",
        100,
        "101"
    )]
    #[case(
        "#.#
...
...",
        100,
        "101"
    )]
    #[case(
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        10,
        "1030"
    )]
    #[case(
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        100,
        "8410"
    )]
    fn test_day11_part2(
        #[case] input: &str,
        #[case] expansion_amount: u32,
        #[case] expected: &str,
    ) {
        assert_eq!(expected, day11_part2_inner(input, expansion_amount));
    }
}
