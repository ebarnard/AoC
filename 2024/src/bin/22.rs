use std::collections::HashMap;

use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1};

use aoc2024::nom::{parse_all, uint32};

const TEST_A: &str = include_str!("../../inputs/22a-test.txt");
const TEST_B: &str = include_str!("../../inputs/22b-test.txt");
const REAL: &str = include_str!("../../inputs/22-real.txt");

fn main() {
    assert_eq!(part1(TEST_A), 37327623);
    assert_eq!(part1(REAL), 16299144133);
    assert_eq!(part2(TEST_B), 23);
    assert_eq!(part2(REAL), 1896);
}

fn parse(input: &str) -> Vec<u32> {
    let lines = separated_list1(tag("\n"), uint32);
    parse_all(input, lines)
}

fn part1(input: &str) -> u64 {
    let secrets = parse(input);

    secrets
        .iter()
        .map(|&secret| (0..2000).fold(secret, |s, _| advance_secret(s)) as u64)
        .sum()
}

fn part2(input: &str) -> u32 {
    let secrets = parse(input);

    let mut pattern_to_monkey = HashMap::new();

    for (i, &first_secret) in secrets.iter().enumerate() {
        let prices = (0..2000).scan(first_secret, |secret, _| {
            let next_secret = advance_secret(*secret);
            let price = next_secret % 10;
            let price_diff = price as i32 - (*secret % 10) as i32;
            *secret = next_secret;
            Some((price, price_diff))
        });

        for ((_, a), (_, b), (_, c), (price, d)) in prices.tuple_windows() {
            let monkey_to_price = pattern_to_monkey
                .entry((a, b, c, d))
                .or_insert(HashMap::new());
            // Only insert the first price
            let _ = monkey_to_price.entry(i).or_insert(price);
        }
    }

    pattern_to_monkey
        .into_values()
        .map(|monkey_to_price| monkey_to_price.into_values().sum())
        .max()
        .unwrap()
}

fn advance_secret(secret: u32) -> u32 {
    let secret = (secret ^ (secret << 6)) & 0xffffff;
    let secret = (secret ^ (secret >> 5)) & 0xffffff;
    let secret = (secret ^ (secret << 11)) & 0xffffff;
    secret
}
