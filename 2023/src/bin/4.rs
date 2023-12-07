use std::collections::HashSet;

use aoc2023::nom::{uint32, ws};
use nom::{
    bytes::complete::tag,
    combinator::complete,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

const TEST: &str = include_str!("../../inputs/4-test.txt");
const REAL: &str = include_str!("../../inputs/4-real.txt");

#[derive(Debug)]
struct Card {
    #[allow(dead_code)]
    id: u32,
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    fn num_matches(&self) -> u32 {
        self.my_numbers.intersection(&self.winning_numbers).count() as u32
    }
}

fn main() {
    assert_eq!(part1(TEST), 13);
    assert_eq!(part1(REAL), 15205);
    assert_eq!(part2(TEST), 30);
    assert_eq!(part2(REAL), 6189740);
}

fn parse(s: &str) -> Vec<Card> {
    fn card(s: &str) -> IResult<&str, Card> {
        let (s, (id, (winning_numbers, my_numbers))) = separated_pair(
            preceded(tag("Card "), ws(uint32)),
            tag(":"),
            separated_pair(many1(ws(uint32)), tag("|"), many1(ws(uint32))),
        )(s)?;

        Ok((
            s,
            Card {
                id,
                winning_numbers: winning_numbers.into_iter().collect(),
                my_numbers: my_numbers.into_iter().collect(),
            },
        ))
    }

    let (_, cards) = complete(separated_list1(tag("\n"), card))(s).unwrap();
    cards
}

fn part1(input: &str) -> u32 {
    let cards = parse(input);

    cards
        .iter()
        .map(|card| {
            let num_matches = card.num_matches();
            if num_matches == 0 {
                0
            } else {
                2u32.pow(num_matches - 1)
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let cards = parse(input);

    let mut cards_won = vec![0; cards.len()];
    (0..cards.len())
        .rev()
        .map(|index| {
            let num_matches = cards[index].num_matches() as usize;
            let num_cards_won = 1 + (1..=num_matches).map(|i| cards_won[index + i]).sum::<u32>();

            cards_won[index] = num_cards_won;
            num_cards_won
        })
        .sum()
}
