use std::collections::HashSet;

use glam::{I64Vec2, IVec2};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, alphanumeric1, newline, space1},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult, Parser,
};

// 1. parse the input
// 2. create a vec of points that are the edges
// 3. use the logic from the previous day where we tested if points were in a polygon. Walk the
//    entire map and count the other points that are in the polygon.
// 4. done
//
// I'm going to parse and store the color information assuming it'll be needed in part 2 but it
// doesn't affect anything for part 1.
#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy)]
struct Instruction<'a> {
    direction: Direction,
    distance: i64,
    color: &'a str,
}

use Direction::*;

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, (direction, distance, color)) = tuple((
        terminated(alpha1, space1).map(|c| match c {
            "R" => East,
            "L" => West,
            "U" => North,
            "D" => South,
            _ => unreachable!("unexpected direction"),
        }),
        terminated(complete::i64, space1),
        delimited(tag("(#"), alphanumeric1, tag(")")),
    ))(input)?;

    Ok((
        input,
        Instruction {
            direction,
            distance,
            color,
        },
    ))
}

fn calculate_lagoon_size(instructions: Vec<Instruction>) -> String {
    let mut dugout: HashSet<IVec2> = HashSet::new();
    let mut position = IVec2 { x: 0, y: 0 };
    let mut min_x: i32 = 0;
    let mut min_y: i32 = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for instruction in instructions {
        dugout.insert(position);
        let (x_range, y_range, direction) = match instruction.direction {
            North => (0..1, -(instruction.distance)..0, IVec2 { x: 0, y: -1 }),
            East => (0..(instruction.distance), 0..1, IVec2 { x: 1, y: 0 }),
            South => (0..1, 0..(instruction.distance), IVec2 { x: 0, y: 1 }),
            West => (-(instruction.distance)..0, 0..1, IVec2 { x: -1, y: 0 }),
        };
        y_range.for_each(|_| {
            x_range.clone().for_each(|_| {
                position = position + direction;
                min_x = min_x.min(position.x);
                min_y = min_y.min(position.y);
                max_x = max_x.max(position.x + 1);
                max_y = max_y.max(position.y + 1);
                dugout.insert(position);
            });
        });
    }

    // draw it (comment this out when running)
    // for y in min_y..max_y {
    //     for x in min_y..max_x {
    //         if dugout
    //             .get(&IVec2 {
    //                 x: x as i32,
    //                 y: y as i32,
    //             })
    //             .is_some()
    //         {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }

    // now walk each line and fill in positions that are in the polygon
    for y in min_y..max_y {
        let mut inside = false;
        for x in min_x..max_x {
            if dugout
                .get(&IVec2 {
                    x: x as i32,
                    y: y as i32,
                })
                .is_some()
            {
                // is there is a cell below, it's vertical or turning down, so toggle it
                let below = IVec2 { x, y: y + 1 };
                if dugout.get(&below).is_some() {
                    inside = !inside;
                }
            } else {
                if inside {
                    dugout.insert(IVec2 {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
    }

    // println!("-----------------");
    //
    let mut count = 0;
    for y in min_y..max_y {
        for x in min_x..max_x {
            if dugout
                .get(&IVec2 {
                    x: x as i32,
                    y: y as i32,
                })
                .is_some()
            {
                count += 1;
                // print!("#");
                //} else {
                // print!(".");
            }
        }
    }

    count.to_string()
}

pub fn day18_part1(input: &str) -> String {
    let (_, instructions) = separated_list1(newline, parse_instruction)(input).unwrap();
    calculate_lagoon_size(instructions)
}

pub fn day18_part2(input: &str) -> String {
    let (_, instructions) = separated_list1(newline, parse_instruction)(input).unwrap();

    // TIL:
    // shoelace algo + border but I didn't get it.
    // needed to look up a bunch of hints here :(
    // most credit: https://github.com/ChristopherBiscardi/advent-of-code/blob/main/2023/rust/day-18/src/part2.rs
    let positions = instructions
        .iter()
        .scan(I64Vec2 { x: 0, y: 0 }, |state, instruction| {
            let distance = &instruction.color[0..5];
            let direction = match &instruction.color[5..6] {
                "0" => I64Vec2 { x: 1, y: 0 },
                "1" => I64Vec2 { x: 0, y: -1 },
                "2" => I64Vec2 { x: -1, y: 0 },
                "3" => I64Vec2 { x: 0, y: 1 },
                _ => unreachable!(),
            };

            let distance = i64::from_str_radix(distance, 16).unwrap();

            *state += direction * distance;
            Some(*state)
        })
        .collect_vec();

    let border = positions
        .iter()
        .tuple_windows()
        .map(|(a, b)| {
            let distance = (*a - *b).abs();
            distance.x + distance.y
        })
        .sum::<i64>()
        + {
            let one = positions.iter().last().unwrap();
            let two = positions.first().unwrap();
            let distance = (*two - *one).abs();
            distance.x + distance.y
        };

    let area = ((positions
        .iter()
        .tuple_windows()
        .map(|(one, two)| one.x * two.y - one.y * two.x)
        .sum::<i64>()
        + border)
        / 2)
    .abs()
        + 1;

    dbg!(area);
    (area + border).to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
",
        "62"
    )]
    // Test going in to negative directions
    #[case(
        "R 6 (#70c710)
U 5 (#0dc571)
L 8 (#5713f0)
D 5 (#d2c081)
R 2 (#d2c081)
",
        "54"
    )]
    fn test_day18_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(day18_part1(input), expected);
    }

    #[rstest]
    #[case(
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
        ",
        "952408144115"
    )]
    #[case(
        "R 6 (#000060)
U 5 (#000053)
L 8 (#000082)
D 5 (#000051)
R 2 (#000020)
",
        "80"
    )]
    fn test_day18_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(day18_part2(input), expected);
    }
}
