use std::{collections::HashMap, ops::ControlFlow};

use num::Integer;

#[derive(Debug)]
struct Map<'a> {
    moves: Vec<char>,
    directions: HashMap<&'a str, (&'a str, &'a str)>,
}

mod parser {
    use super::*;
    use nom::{
        bytes::complete::tag,
        character::complete::{alphanumeric1, newline},
        multi::{many1, separated_list1},
        sequence::{preceded, separated_pair, terminated},
        IResult,
    };

    fn moves(input: &str) -> IResult<&str, Vec<char>> {
        let (input, moves) = alphanumeric1(input)?;
        Ok((input, moves.chars().collect::<Vec<_>>()))
    }

    // AAA = (BBB, CCC)
    fn direction(input: &str) -> IResult<&str, (&str, (&str, &str))> {
        let (input, pieces) = separated_pair(
            alphanumeric1,
            tag(" = "),
            separated_pair(
                preceded(tag("("), alphanumeric1),
                tag(", "),
                terminated(alphanumeric1, tag(")")),
            ),
        )(input)?;
        Ok((input, pieces))
    }

    pub fn parse(input: &str) -> IResult<&str, Map> {
        let (input, (moves, directions)) =
            separated_pair(moves, many1(newline), separated_list1(newline, direction))(input)?;
        let directions = directions
            .into_iter()
            .collect::<HashMap<&str, (&str, &str)>>();
        Ok((input, Map { moves, directions }))
    }
}

fn steps_to_find_from(map: &Map, position: &str, end: &str) -> usize {
    let mut position = position;
    let mut steps = 0;
    loop {
        map.moves.iter().try_for_each(|m| {
            if position.ends_with(end) {
                return ControlFlow::Break(());
            }
            steps += 1;
            let dest = map.directions.get(position).unwrap();
            match m {
                'L' => {
                    position = dest.0;
                }
                'R' => {
                    position = dest.1;
                }
                _ => unreachable!(),
            }
            ControlFlow::Continue(())
        });
        if position.ends_with(end) {
            break;
        }
    }
    steps
}

pub fn day8_part1(input: &str) -> String {
    let (remainder, map) = parser::parse(input).unwrap();
    assert!(remainder.trim().len() == 0);

    let steps = steps_to_find_from(&map, "AAA", "ZZZ");
    steps.to_string()
}

pub fn day8_part2(input: &str) -> String {
    let (remainder, map) = parser::parse(input).unwrap();
    assert!(remainder.trim().len() == 0);

    // Break part 1 out to a helper "steps_to_find_from"
    // that takes a start and ends_with.
    // Find the path for each start and then get the LCM of those
    map.directions
        .keys()
        .filter_map(|position| {
            if position.chars().last().unwrap() == 'A' {
                Some(steps_to_find_from(&map, position, "Z"))
            } else {
                None
            }
        })
        .fold(1 as usize, |a, b| a.lcm(&b))
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        "2"
    )]
    #[case(
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        "6"
    )]
    fn test_day8_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, day8_part1(input));
    }

    #[rstest]
    #[case(
        "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        "6"
    )]
    fn test_day8_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(expected, day8_part2(input));
    }
}
