use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, multispace0, multispace1, space0, space1},
    multi::{many0, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> usize {
    let mut cards: BTreeMap<usize, usize> = BTreeMap::new();
    input.lines().for_each(|line| {
        let (_, card) = parse_card(line).unwrap();
        let card_id = card.id.parse::<usize>().unwrap();

        cards.insert(card_id, 1);
    });

    input.lines().for_each(|line| {
        let (_, card) = parse_card(line).unwrap();
        let card_id = card.id.parse::<usize>().unwrap();
        let times_to_process = cards.get(&card_id).copied().unwrap_or(1);

        let winning_card_nums = card
            .card_numbers
            .into_iter()
            .filter(|card_num| card.winning_numbers.contains(card_num))
            .collect::<Vec<u32>>();

        winning_card_nums
            .into_iter()
            .enumerate()
            .for_each(|(i, _)| {
                let card_num = card_id + i + 1;

                cards.entry(card_num).and_modify(|v| {
                    *v = *v + times_to_process;
                });
            });
    });

    dbg!(&cards);

    cards.values().sum()
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (remaining, id) = preceded(tuple((tag("Card"), multispace0)), digit1)(input)?;
    let (remaining, numbers) = preceded(
        tag(":"),
        preceded(
            multispace1,
            separated_list1(tuple((tag("|"), multispace0)), parse_numbers),
        ),
    )(remaining)?;

    let winning_numbers: Vec<u32> = numbers
        .get(0)
        .into_iter()
        .flat_map(|winning_numbers| {
            winning_numbers
                .iter()
                .map(|(num, _)| *num)
                .collect::<Vec<u32>>()
        })
        .collect();

    let card_numbers: Vec<u32> = numbers
        .get(1)
        .into_iter()
        .flat_map(|card_numbers| {
            card_numbers
                .into_iter()
                .map(|(num, _)| *num)
                .collect::<Vec<u32>>()
        })
        .collect();

    Ok((
        remaining,
        Card {
            id,
            winning_numbers,
            card_numbers,
        },
    ))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<(u32, &str)>> {
    many0(tuple((complete::u32, multispace0)))(input)
}

#[derive(Debug)]
struct Card<'a> {
    id: &'a str,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 30);
    }
}
