use self::parser::games;

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<(u32, u32, u32)>,
}

mod parser {
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, multispace1, newline, u32},
        multi::{separated_list0, separated_list1},
        sequence::{preceded, separated_pair, terminated},
        IResult,
    };

    use super::Game;

    fn color(input: &str) -> IResult<&str, (u32, &str)> {
        let (input, color) = separated_pair(u32, multispace1, alpha1)(input)?;
        Ok((input, color))
    }

    fn round(input: &str) -> IResult<&str, Vec<(u32, &str)>> {
        let (input, colors) = separated_list0(tag(", "), color)(input)?;
        Ok((input, colors))
    }

    fn game(input: &str) -> IResult<&str, Game> {
        let (input, id) = terminated(preceded(tag("Game "), u32), tag(": "))(input)?;
        let (input, rounds) = separated_list1(tag("; "), round)(input)?;
        let mut game = Game {
            id,
            rounds: Vec::new(),
        };

        rounds.iter().for_each(|parsed_round| {
            let mut round = (0, 0, 0);
            parsed_round.iter().for_each(|color| match color.1 {
                "red" => round.0 = color.0,
                "green" => round.1 = color.0,
                "blue" => round.2 = color.0,
                _ => panic!(),
            });
            game.rounds.push(round)
        });

        Ok((input, game))
    }

    pub fn games(input: &str) -> IResult<&str, Vec<Game>> {
        let (input, games) = separated_list1(newline, game)(input)?;
        Ok((input, games))
    }
}

pub fn day2_part1(input: &str) -> String {
    let (_, games) = games(input).unwrap();
    let max_possible = (12, 13, 14);
    games
        .iter()
        .filter_map(|game| {
            let all = game.rounds.iter().all(|round| {
                if round.0 <= max_possible.0
                    && round.1 <= max_possible.1
                    && round.2 <= max_possible.2
                {
                    true
                } else {
                    false
                }
            });
            if all {
                Some(game.id)
            } else {
                None
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn day2_part2(input: &str) -> String {
    let (_, games) = games(input).unwrap();
    games
        .iter()
        .map(|game| {
            let sums = game.rounds.iter().fold((0, 0, 0), |acc, round| {
                (acc.0.max(round.0), acc.1.max(round.1), acc.2.max(round.2))
            });
            sums.0 * sums.1 * sums.2
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_day2_part1() {
        assert_eq!("8", day2_part1(INPUT));
    }

    #[test]
    fn test_day2_part2() {
        assert_eq!("2286", day2_part2(INPUT));
    }
}
