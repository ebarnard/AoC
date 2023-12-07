use aoc2023::nom::{uint32, ws};
use nom::{
    bytes::complete::{tag, take},
    combinator::{complete, map},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const TEST: &str = include_str!("../../inputs/7-test.txt");
const REAL: &str = include_str!("../../inputs/7-real.txt");

#[derive(Debug)]
struct Hand {
    cards: [char; 5],
    bid: u32,
}

fn main() {
    assert_eq!(part1(TEST), 6440);
    assert_eq!(part1(REAL), 252295678);
    assert_eq!(part2(TEST), 5905);
    assert_eq!(part2(REAL), 250577259);
}

fn part1(input: &str) -> u32 {
    let hands = parse(input);

    fn hand_sort_key(hand: &Hand) -> (u8, [u8; 5]) {
        let strength_order = &[
            '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
        ];

        let card_strengths = card_strengths(&hand.cards, strength_order);
        let mut card_counts = card_counts(&card_strengths);

        card_counts.sort_unstable();

        let hand_strength = hand_strength(card_counts[12], card_counts[11]);

        (hand_strength, card_strengths)
    }

    total_winnings(&hands, hand_sort_key)
}

fn part2(input: &str) -> u32 {
    let hands = parse(input);

    fn hand_sort_key(hand: &Hand) -> (u8, [u8; 5]) {
        let strength_order = &[
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ];

        let card_strengths = card_strengths(&hand.cards, strength_order);
        let mut card_counts = card_counts(&card_strengths);

        // Remove and save jokers.
        let num_jokers = card_counts[0];
        card_counts[0] = 0;

        card_counts.sort_unstable();

        // Jokers convert to the most populous card.
        card_counts[12] += num_jokers;
        let hand_strength = hand_strength(card_counts[12], card_counts[11]);

        (hand_strength, card_strengths)
    }

    total_winnings(&hands, hand_sort_key)
}

fn parse(s: &str) -> Vec<Hand> {
    fn hand(s: &str) -> IResult<&str, Hand> {
        map(
            separated_pair(take(5u32), tag(" "), ws(uint32)),
            |(cards, bid)| Hand {
                cards: cards.chars().collect::<Vec<_>>().try_into().unwrap(),
                bid,
            },
        )(s)
    }

    let (_, hands) = complete(separated_list1(tag("\n"), hand))(s).unwrap();
    hands
}

fn card_strengths(cards: &[char; 5], strength_order: &[char]) -> [u8; 5] {
    cards.map(|card| {
        strength_order
            .iter()
            .enumerate()
            .filter(|&(_, &c)| c == card)
            .next()
            .unwrap()
            .0 as u8
    })
}

fn card_counts(card_strengths: &[u8; 5]) -> [u8; 13] {
    let mut card_counts = [0u8; 13];
    for &card in card_strengths {
        card_counts[usize::from(card)] += 1;
    }
    card_counts
}

fn hand_strength(max_card_count: u8, second_max_card_count: u8) -> u8 {
    match (max_card_count, second_max_card_count) {
        (5, 0) => 6,
        (4, 1) => 5,
        (3, 2) => 4,
        (3, 1) => 3,
        (2, 2) => 2,
        (2, 1) => 1,
        (1, 1) => 0,
        v => unreachable!("{:?}", v),
    }
}

fn total_winnings(hands: &[Hand], sort_key: impl Fn(&Hand) -> (u8, [u8; 5])) -> u32 {
    let mut hands: Vec<_> = hands.iter().collect();
    hands.sort_by_cached_key(|&hand| sort_key(hand));
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}
