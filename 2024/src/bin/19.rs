use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::alpha1, multi::separated_list1,
    sequence::separated_pair,
};

use aoc2024::nom::parse_all;

const TEST: &str = include_str!("../../inputs/19-test.txt");
const REAL: &str = include_str!("../../inputs/19-real.txt");

fn main() {
    assert_eq!(part1(TEST), 6);
    assert_eq!(part1(REAL), 216);
    assert_eq!(part2(TEST), 16);
    assert_eq!(part2(REAL), 603191454138773);
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let patterns = separated_list1(tag(", "), alpha1::<_, ()>);
    let designs = separated_list1(tag("\n"), alpha1);
    let parser = separated_pair(patterns, tag("\n\n"), designs);

    parse_all(input, parser)
}

fn part1(input: &str) -> u32 {
    let (patterns, designs) = parse(input);

    let mut memo = HashMap::new();
    designs
        .iter()
        .filter(|p| count(p, &patterns, &mut memo) > 0)
        .count() as u32
}

fn part2(input: &str) -> u64 {
    let (patterns, designs) = parse(input);

    let mut memo = HashMap::new();
    designs.iter().map(|p| count(p, &patterns, &mut memo)).sum()
}

fn count<'a, 'b>(
    design: &'a str,
    patterns: &'b [&'a str],
    memo: &'b mut HashMap<&'a str, u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }

    if let Some(&count) = memo.get(design) {
        return count;
    }

    let count = patterns
        .iter()
        .filter(|&p| design.starts_with(p))
        .map(|p| count(&design[p.len()..], patterns, memo))
        .sum();

    memo.insert(design, count);
    count
}
