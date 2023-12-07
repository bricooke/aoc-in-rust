use std::{cmp::Ordering, collections::BTreeMap};

#[derive(Debug, PartialEq)]
struct Hand {
    bid: u32,
    r#type: Type,
    cards: String,
}

fn build_hand(bid: u32, cards: String, part2: bool) -> Hand {
    let cards = cards
        .replace("A", "Z")
        .replace("K", "Y")
        .replace("Q", "X")
        .replace("J", if part2 { "1" } else { "W" })
        .replace("T", "V");
    let t = type_from_cards(&cards, part2);
    Hand {
        bid,
        r#type: t,
        cards,
    }
}
impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // This could be tidied up, of course.
        if self.r#type == other.r#type {
            self.cards.cmp(&other.cards)
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

fn type_from_cards(cards: &String, part2: bool) -> Type {
    let cards = cards.chars().collect::<Vec<char>>();
    let counts = cards
        .iter()
        .map(|card| (*card, cards.iter().filter(|c| **c == *card).count()))
        .collect::<BTreeMap<char, usize>>();

    let joker_count = if part2 {
        counts.get(&'1').or(Some(&0)).unwrap()
    } else {
        &0
    };
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

fn hands(input: &str, part2: bool) -> Vec<Hand> {
    input
        .lines()
        .map(|hand| {
            let line = hand.split(" ").collect::<Vec<&str>>();
            build_hand(line[1].parse::<u32>().unwrap(), line[0].to_string(), part2)
        })
        .collect::<Vec<_>>()
}

pub fn day7_part1(input: &str) -> String {
    let mut hands = hands(input, false);
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum::<u32>()
        .to_string()
}

pub fn day7_part2(input: &str) -> String {
    let mut hands = hands(input, true);
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
    fn test_day7_part1() {
        assert_eq!("6440", day7_part1(INPUT));
    }

    #[test]
    fn test_day7_part2() {
        assert_eq!("5905", day7_part2(INPUT));
    }

    #[test]
    fn test_day7_part2_jokers() {
        // Note the 1s here are jokers since type_from_cards runs post character conversion
        assert_eq!(FiveOfAKind, type_from_cards(&"1111K".to_string(), true));
        assert_eq!(FiveOfAKind, type_from_cards(&"KKK1K".to_string(), true));
        assert_eq!(FiveOfAKind, type_from_cards(&"1KKK1".to_string(), true));
        assert_eq!(OnePair, type_from_cards(&"23451".to_string(), true));
        assert_eq!(ThreeOfAKind, type_from_cards(&"11KQT".to_string(), true));
        assert_eq!(ThreeOfAKind, type_from_cards(&"1KKQT".to_string(), true));
        assert_eq!(FourOfAKind, type_from_cards(&"11K1T".to_string(), true));
        assert_eq!(FourOfAKind, type_from_cards(&"1KK1T".to_string(), true));
        assert_eq!(FourOfAKind, type_from_cards(&"1KQQ1".to_string(), true));
        assert_eq!(ThreeOfAKind, type_from_cards(&"1KTQ1".to_string(), true));
        assert_eq!(FullHouse, type_from_cards(&"1KKQQ".to_string(), true));
        assert_eq!(FourOfAKind, type_from_cards(&"1K1QQ".to_string(), true));
        assert_eq!(ThreeOfAKind, type_from_cards(&"1KKQT".to_string(), true));
        assert_eq!(FiveOfAKind, type_from_cards(&"KKK11".to_string(), true));
        assert_eq!(FourOfAKind, type_from_cards(&"KQK11".to_string(), true));
        assert_eq!(FourOfAKind, type_from_cards(&"KQKK1".to_string(), true));
    }
}
