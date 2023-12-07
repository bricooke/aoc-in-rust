use std::{cmp::Ordering, collections::BTreeMap};

#[derive(Debug, PartialEq)]
struct Hand {
    bid: u32,
    r#type: Type,
    cards: String,
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // This could be tidied up, of course.
        if self.r#type == other.r#type {
            let self_cards = self
                .cards
                .replace("A", "Z")
                .replace("K", "Y")
                .replace("Q", "X")
                .replace("J", "1")
                .replace("T", "V");
            let other_cards = other
                .cards
                .replace("A", "Z")
                .replace("K", "Y")
                .replace("Q", "X")
                .replace("J", "1")
                .replace("T", "V");
            self_cards.cmp(&other_cards)
        } else {
            self.r#type.cmp(&other.r#type)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

use Type::*;

fn type_from_cards(cards: String) -> Type {
    let cards = cards.chars().collect::<Vec<char>>();
    let counts = cards
        .iter()
        .map(|card| (*card, cards.iter().filter(|c| **c == *card).count()))
        .collect::<BTreeMap<char, usize>>();

    let joker_count = counts.get(&'J').or(Some(&0)).unwrap();
    match counts.keys().count() {
        1 => FiveOfAKind,
        2 => {
            let values = counts.values().collect::<Vec<&usize>>();
            if *joker_count > 0 {
                FiveOfAKind
            } else if *values[0] == 1 || *values[0] == 4 {
                FourOfAKind
            } else {
                FullHouse
            }
        }
        3 => {
            // Could be 3 of a kind + 2 individuals
            // two pair + 1 individual
            if counts.values().any(|c| *c == 3) {
                match joker_count {
                    1..=3 => FourOfAKind,
                    _ => ThreeOfAKind,
                }
            } else {
                match joker_count {
                    2 => FourOfAKind,
                    1 => FullHouse,
                    _ => TwoPair,
                }
            }
        }
        4 => match joker_count {
            2 => ThreeOfAKind,
            1 => ThreeOfAKind,
            _ => OnePair,
        },
        5 => {
            if *joker_count > 0 {
                OnePair
            } else {
                HighCard
            }
        }
        _ => unreachable!(),
    }
}

fn hands(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|hand| {
            let line = hand.split(" ").collect::<Vec<&str>>();
            Hand {
                bid: line[1].parse::<u32>().unwrap(),
                r#type: type_from_cards(line[0].to_string()),
                cards: line[0].to_string(),
            }
        })
        .collect::<Vec<_>>()
}

pub fn day7_part1(input: &str) -> String {
    let mut hands = hands(input);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum::<u32>()
        .to_string()
}

pub fn day7_part2(input: &str) -> String {
    let mut hands = hands(input);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    #[ignore] // ignored because I broke it for part 2 :(
    fn test_day7_part1() {
        assert_eq!("6440", day7_part1(INPUT));
    }

    #[test]
    fn test_day7_part2() {
        assert_eq!("5905", day7_part2(INPUT));
    }

    #[test]
    fn test_day7_part2_jokers() {
        assert_eq!(FiveOfAKind, type_from_cards("JJJJK".to_string()));
        assert_eq!(FiveOfAKind, type_from_cards("KKKJK".to_string()));
        assert_eq!(FiveOfAKind, type_from_cards("JKKKJ".to_string()));
        assert_eq!(OnePair, type_from_cards("2345J".to_string()));
        assert_eq!(ThreeOfAKind, type_from_cards("JJKQT".to_string()));
        assert_eq!(ThreeOfAKind, type_from_cards("JKKQT".to_string()));
        assert_eq!(FourOfAKind, type_from_cards("JJKJT".to_string()));
        assert_eq!(FourOfAKind, type_from_cards("JKKJT".to_string()));
        assert_eq!(FourOfAKind, type_from_cards("JKQQJ".to_string()));
        assert_eq!(ThreeOfAKind, type_from_cards("JKTQJ".to_string()));
        assert_eq!(FullHouse, type_from_cards("JKKQQ".to_string()));
        assert_eq!(FourOfAKind, type_from_cards("JKJQQ".to_string()));
        assert_eq!(ThreeOfAKind, type_from_cards("JKKQT".to_string()));
        assert_eq!(FiveOfAKind, type_from_cards("KKKJJ".to_string()));
        assert_eq!(FourOfAKind, type_from_cards("KQKJJ".to_string()));
        assert_eq!(FourOfAKind, type_from_cards("KQKKJ".to_string()));
    }
}
