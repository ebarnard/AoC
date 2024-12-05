use std::collections::HashMap;

use nom::{bytes::complete::tag, multi::separated_list0, sequence::separated_pair};

use aoc2024::nom::{parse_all, uint32, ws};

const TEST: &str = include_str!("../../inputs/1-test.txt");
const REAL: &str = include_str!("../../inputs/1-real.txt");

fn main() {
    assert_eq!(part1(TEST), 11);
    assert_eq!(part1(REAL), 2742123);
    assert_eq!(part2(TEST), 31);
    assert_eq!(part2(REAL), 21328497);
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let line = separated_pair(uint32, ws, uint32);
    let lines = separated_list0(tag("\n"), line);

    let parsed = parse_all(input, lines);
    parsed.into_iter().unzip()
}

fn part1(input: &str) -> u32 {
    let (mut left, mut right) = parse(input);

    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn part2(input: &str) -> u32 {
    let (left, right) = parse(input);

    let right = right.into_iter().fold(HashMap::new(), |mut counts, r| {
        *counts.entry(r).or_insert(0u32) += 1;
        counts
    });

    left.into_iter()
        .map(|l| l * right.get(&l).copied().unwrap_or(0))
        .sum()
}
