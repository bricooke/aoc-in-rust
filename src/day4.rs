use std::collections::HashMap;

use self::parser::cards;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Card {
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

mod parser {
    use nom::{
        bytes::complete::tag,
        character::complete::{self, newline},
        multi::{many1, separated_list1},
        sequence::{preceded, separated_pair, terminated},
        IResult,
    };

    use super::Card;

    fn card(input: &str) -> IResult<&str, Card> {
        let (input, _) = terminated(
            terminated(
                preceded(terminated(tag("Card"), many1(tag(" "))), complete::u32),
                tag(":"),
            ),
            many1(tag(" ")),
        )(input)?;
        let (input, numbers) = separated_pair(
            separated_list1(many1(tag(" ")), complete::u32),
            separated_pair(many1(tag(" ")), tag("|"), many1(tag(" "))),
            separated_list1(many1(tag(" ")), complete::u32),
        )(input)?;

        Ok((
            input,
            Card {
                winning_numbers: numbers.0,
                my_numbers: numbers.1,
            },
        ))
    }

    pub fn cards(input: &str) -> Vec<Card> {
        let (remainder, cards) = separated_list1(newline, card)(input).unwrap();
        assert_eq!("", remainder.trim());
        cards
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_card() {
            assert_eq!(
                Card {
                    winning_numbers: vec![41, 48, 83, 86, 17],
                    my_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
                },
                card("Card  1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53")
                    .unwrap()
                    .1
            );
        }

        #[test]
        fn test_card_with_space() {
            assert_eq!(
                Card {
                    winning_numbers: vec![1, 21, 53, 59, 44],
                    my_numbers: vec![9, 82, 63, 72, 16, 21, 14, 1]
                },
                card("Card 3:  1 21 53 59 44 |  9 82 63 72 16 21 14  1")
                    .unwrap()
                    .1
            );
        }
    }
}

pub fn day4_part1(input: &str) -> String {
    let cards = cards(input);
    cards
        .iter()
        .map(|card| {
            let mut score = 0;
            card.winning_numbers.iter().for_each(|w| {
                if card.my_numbers.contains(w) {
                    if score == 0 {
                        score = 1
                    } else {
                        score *= 2
                    }
                }
            });
            score
        })
        .sum::<u32>()
        .to_string()
}

pub fn day4_part2(input: &str) -> String {
    let cards = cards(input);
    let mut copies_by_index = cards
        .iter()
        .enumerate()
        .map(|(i, _)| (i, 0))
        .collect::<HashMap<usize, u32>>();

    for (index, card) in cards.iter().enumerate() {
        let mut wins = 0;
        card.winning_numbers.iter().for_each(|w| {
            if card.my_numbers.contains(w) {
                wins += 1;
            }
        });

        let additional_runs = *copies_by_index.get(&index).unwrap();

        for _ in 0..=additional_runs {
            for i in (index + 1)..(index + 1 + wins) {
                match copies_by_index.get_mut(&i) {
                    Some(count) => *count += 1,
                    None => {
                        unreachable!("The HashMap should have been seeded with 0s for all indexes")
                    }
                }
            }
        }
    }

    copies_by_index
        .iter()
        .map(|(_, v)| v + 1)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_day4_part1() {
        assert_eq!("13", day4_part1(INPUT));
    }

    #[test]
    fn test_day4_part2() {
        assert_eq!("30", day4_part2(INPUT));
    }
}
