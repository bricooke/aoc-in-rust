use std::collections::HashMap;

use glam::IVec2;

enum Direction {
    North,
    West,
    South,
    East,
}

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

fn shift_rocks(map: &mut HashMap<IVec2, Rock>, direction: Direction) {}

pub fn day14_part2(input: &str) -> String {
    todo!();
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
        "64"
    )]
    fn test_day14_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(day14_part2(input), expected);
    }
}
