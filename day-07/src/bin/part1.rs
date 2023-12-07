use std::{cmp::Ordering, collections::HashMap};

use nom::{
    character::complete::{self, alphanumeric1, space1},
    sequence::separated_pair,
    IResult,
};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let mut hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|line| {
            let (_, (hand, bet)) = parse(line).unwrap();

            (hand, bet)
        })
        .collect();

    hands.sort_by(|(hand1, _), (hand2, _)| hand2.cmp(hand1));

    hands.reverse();

    let winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, bet))| {
            let rank = i as u32 + 1;

            *bet * rank
        })
        .sum();

    winnings
}

fn parse<'a>(input: &'a str) -> IResult<&'a str, (Hand<'a>, u32)> {
    separated_pair(parse_hand, space1, complete::u32)(input)
}

fn parse_hand<'a>(input: &'a str) -> IResult<&'a str, Hand<'a>> {
    let (remaining, raw_hand) = alphanumeric1(input)?;

    let hand = Hand::new(raw_hand);

    Ok((remaining, hand))
}

#[derive(Debug, PartialEq)]
enum Hand<'a> {
    FiveOfAKind(&'a str),
    FourOfAKind(&'a str),
    FullHouse(&'a str),
    ThreeOfAKind(&'a str),
    TwoPair(&'a str),
    OnePair(&'a str),
    HighCard(&'a str),
}

impl<'a> Hand<'a> {
    fn new(raw_hand: &'a str) -> Hand<'a> {
        let mut card_counts: HashMap<char, u32> = HashMap::new();

        raw_hand.chars().for_each(|char| {
            *card_counts.entry(char).or_insert(0) += 1;
        });

        match card_counts.len() {
            1 => Hand::FiveOfAKind(raw_hand),
            2 => {
                let max_count = card_counts.iter().map(|(_, count)| count).max().unwrap();

                match *max_count {
                    4 => Hand::FourOfAKind(raw_hand),
                    3 => Hand::FullHouse(raw_hand),
                    _ => Hand::HighCard(raw_hand),
                }
            }
            3 => {
                let max_count = card_counts.iter().map(|(_, count)| count).max().unwrap();

                match *max_count {
                    3 => Hand::ThreeOfAKind(raw_hand),
                    2 => Hand::TwoPair(raw_hand),
                    _ => Hand::HighCard(raw_hand),
                }
            }
            4 => Hand::OnePair(raw_hand),
            _ => Hand::HighCard(raw_hand),
        }
    }

    fn cmp(&self, other: &Hand) -> Ordering {
        let ordering = match (self, other) {
            (Hand::FiveOfAKind(_), Hand::FiveOfAKind(_)) => Ordering::Equal,
            (Hand::FullHouse(_), Hand::FullHouse(_)) => Ordering::Equal,
            (Hand::FourOfAKind(_), Hand::FourOfAKind(_)) => Ordering::Equal,
            (Hand::ThreeOfAKind(_), Hand::ThreeOfAKind(_)) => Ordering::Equal,
            (Hand::TwoPair(_), Hand::TwoPair(_)) => Ordering::Equal,
            (Hand::OnePair(_), Hand::OnePair(_)) => Ordering::Equal,
            (Hand::HighCard(_), Hand::HighCard(_)) => Ordering::Equal,

            (Hand::FiveOfAKind(_), Hand::FourOfAKind(_)) => Ordering::Greater,
            (Hand::FiveOfAKind(_), Hand::FullHouse(_)) => Ordering::Greater,
            (Hand::FiveOfAKind(_), Hand::ThreeOfAKind(_)) => Ordering::Greater,
            (Hand::FiveOfAKind(_), Hand::TwoPair(_)) => Ordering::Greater,
            (Hand::FiveOfAKind(_), Hand::OnePair(_)) => Ordering::Greater,
            (Hand::FiveOfAKind(_), Hand::HighCard(_)) => Ordering::Greater,

            (Hand::FourOfAKind(_), Hand::FullHouse(_)) => Ordering::Greater,
            (Hand::FourOfAKind(_), Hand::ThreeOfAKind(_)) => Ordering::Greater,
            (Hand::FourOfAKind(_), Hand::TwoPair(_)) => Ordering::Greater,
            (Hand::FourOfAKind(_), Hand::OnePair(_)) => Ordering::Greater,
            (Hand::FourOfAKind(_), Hand::HighCard(_)) => Ordering::Greater,

            (Hand::FullHouse(_), Hand::ThreeOfAKind(_)) => Ordering::Greater,
            (Hand::FullHouse(_), Hand::TwoPair(_)) => Ordering::Greater,
            (Hand::FullHouse(_), Hand::OnePair(_)) => Ordering::Greater,
            (Hand::FullHouse(_), Hand::HighCard(_)) => Ordering::Greater,

            (Hand::ThreeOfAKind(_), Hand::TwoPair(_)) => Ordering::Greater,
            (Hand::ThreeOfAKind(_), Hand::OnePair(_)) => Ordering::Greater,
            (Hand::ThreeOfAKind(_), Hand::HighCard(_)) => Ordering::Greater,

            (Hand::TwoPair(_), Hand::OnePair(_)) => Ordering::Greater,
            (Hand::TwoPair(_), Hand::HighCard(_)) => Ordering::Greater,

            (Hand::OnePair(_), Hand::HighCard(_)) => Ordering::Greater,

            (Hand::FourOfAKind(_), Hand::FiveOfAKind(_)) => Ordering::Less,
            (Hand::FullHouse(_), Hand::FiveOfAKind(_)) => Ordering::Less,
            (Hand::ThreeOfAKind(_), Hand::FiveOfAKind(_)) => Ordering::Less,
            (Hand::TwoPair(_), Hand::FiveOfAKind(_)) => Ordering::Less,
            (Hand::OnePair(_), Hand::FiveOfAKind(_)) => Ordering::Less,
            (Hand::HighCard(_), Hand::FiveOfAKind(_)) => Ordering::Less,

            (Hand::FullHouse(_), Hand::FourOfAKind(_)) => Ordering::Less,
            (Hand::ThreeOfAKind(_), Hand::FourOfAKind(_)) => Ordering::Less,
            (Hand::TwoPair(_), Hand::FourOfAKind(_)) => Ordering::Less,
            (Hand::OnePair(_), Hand::FourOfAKind(_)) => Ordering::Less,
            (Hand::HighCard(_), Hand::FourOfAKind(_)) => Ordering::Less,

            (Hand::ThreeOfAKind(_), Hand::FullHouse(_)) => Ordering::Less,
            (Hand::TwoPair(_), Hand::FullHouse(_)) => Ordering::Less,
            (Hand::OnePair(_), Hand::FullHouse(_)) => Ordering::Less,
            (Hand::HighCard(_), Hand::FullHouse(_)) => Ordering::Less,

            (Hand::TwoPair(_), Hand::ThreeOfAKind(_)) => Ordering::Less,
            (Hand::OnePair(_), Hand::ThreeOfAKind(_)) => Ordering::Less,
            (Hand::HighCard(_), Hand::ThreeOfAKind(_)) => Ordering::Less,

            (Hand::OnePair(_), Hand::TwoPair(_)) => Ordering::Less,
            (Hand::HighCard(_), Hand::TwoPair(_)) => Ordering::Less,

            (Hand::HighCard(_), Hand::OnePair(_)) => Ordering::Less,

            _ => Ordering::Equal,
        };

        match ordering {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let self_values = match *self {
                    Self::FiveOfAKind(val) => val,
                    Self::FullHouse(val) => val,
                    Self::FourOfAKind(val) => val,
                    Self::ThreeOfAKind(val) => val,
                    Self::TwoPair(val) => val,
                    Self::OnePair(val) => val,
                    Self::HighCard(val) => val,
                }
                .as_bytes();
                let other_values = match *other {
                    Hand::FiveOfAKind(val) => val,
                    Hand::FullHouse(val) => val,
                    Hand::FourOfAKind(val) => val,
                    Hand::ThreeOfAKind(val) => val,
                    Hand::TwoPair(val) => val,
                    Hand::OnePair(val) => val,
                    Hand::HighCard(val) => val,
                }
                .as_bytes();

                for i in 0..self_values.len() {
                    let self_val = self_values[i] as char;
                    let other_val = other_values[i] as char;

                    if card_value(self_val) > card_value(other_val) {
                        return Ordering::Greater;
                    } else if card_value(self_val) < card_value(other_val) {
                        return Ordering::Less;
                    } else {
                        continue;
                    }
                }

                Ordering::Equal
            }
        }
    }
}

fn card_value(card: char) -> u32 {
    let values: HashMap<char, u32> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);

    *values.get(&card).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, 6440);
    }

    #[test]
    fn make_hand() {
        let hand1 = Hand::new("32T3K");
        assert_eq!(hand1, Hand::OnePair("32T3K"));
        let hand2 = Hand::new("T55J5");
        assert_eq!(hand2, Hand::ThreeOfAKind("T55J5"));
        let hand3 = Hand::new("KK677");
        assert_eq!(hand3, Hand::TwoPair("KK677"));
        let hand4 = Hand::new("KTJJT");
        assert_eq!(hand4, Hand::TwoPair("KTJJT"));
        let hand5 = Hand::new("QQQJA");
        assert_eq!(hand5, Hand::ThreeOfAKind("QQQJA"));
        let hand6 = Hand::new("99T99");
        assert_eq!(hand6, Hand::FourOfAKind("99T99"));
        let hand7 = Hand::new("KAAAA");
        assert_eq!(hand7, Hand::FourOfAKind("KAAAA"));
    }
}
