use std::collections::HashSet;

use glam::IVec2;
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
#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct Instruction<'a> {
    direction: Direction,
    distance: usize,
    _color: &'a str,
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
        terminated(complete::u32, space1),
        delimited(tag("(#"), alphanumeric1, tag(")")),
    ))(input)?;

    Ok((
        input,
        Instruction {
            direction,
            distance: distance as usize,
            _color: color,
        },
    ))
}

pub fn day18_part1(input: &str) -> String {
    let (_, instructions) = separated_list1(newline, parse_instruction)(input).unwrap();
    let mut dugout: HashSet<IVec2> = HashSet::new();
    let mut position = IVec2 { x: 0, y: 0 };
    let mut min_x: i32 = 0;
    let mut min_y: i32 = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for instruction in instructions {
        dugout.insert(position);
        let (x_range, y_range, direction) = match instruction.direction {
            North => (
                0..1,
                -(instruction.distance as i32)..0,
                IVec2 { x: 0, y: -1 },
            ),
            East => (0..(instruction.distance as i32), 0..1, IVec2 { x: 1, y: 0 }),
            South => (0..1, 0..(instruction.distance as i32), IVec2 { x: 0, y: 1 }),
            West => (
                -(instruction.distance as i32)..0,
                0..1,
                IVec2 { x: -1, y: 0 },
            ),
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
    for y in min_y..max_y {
        for x in min_y..max_x {
            if dugout
                .get(&IVec2 {
                    x: x as i32,
                    y: y as i32,
                })
                .is_some()
            {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

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

    println!("-----------------");

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
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    count.to_string()
}

pub fn day18_part2(_input: &str) -> String {
    todo!();
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

    #[test]
    #[ignore]
    fn test_day18_part2() {
        assert_eq!("0", day18_part2(""));
    }
}
