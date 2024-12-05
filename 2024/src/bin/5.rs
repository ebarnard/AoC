use std::cmp::Ordering;

use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1, sequence::separated_pair};

use aoc2024::nom::{parse_all, uint32};

const TEST: &str = include_str!("../../inputs/5-test.txt");
const REAL: &str = include_str!("../../inputs/5-real.txt");

fn main() {
    assert_eq!(part1(TEST), 143);
    assert_eq!(part1(REAL), 5166);
    assert_eq!(part2(TEST), 123);
    assert_eq!(part2(REAL), 4679);
}

fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let ordering_rule = separated_pair(uint32, tag("|"), uint32);
    let ordering_rules = separated_list1(tag("\n"), ordering_rule);

    let pages = separated_list1(tag(","), uint32);
    let updates = separated_list1(tag("\n"), pages);

    let parser = separated_pair(ordering_rules, tag("\n\n"), updates);
    parse_all(input, parser)
}

fn part1(input: &str) -> u32 {
    let (ordering_rules, updates) = parse(input);

    updates
        .into_iter()
        .filter(|pages| &order_pages(pages, &ordering_rules) == pages)
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

fn part2(input: &str) -> u32 {
    let (ordering_rules, updates) = parse(input);

    updates
        .into_iter()
        .filter_map(|pages| {
            let ordered = order_pages(&pages, &ordering_rules);
            if ordered == pages {
                None
            } else {
                Some(ordered)
            }
        })
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

fn order_pages(pages: &[u32], ordering_rules: &[(u32, u32)]) -> Vec<u32> {
    pages
        .iter()
        .copied()
        .sorted_by(|&x, &y| {
            if ordering_rules.contains(&(x, y)) {
                Ordering::Less
            } else if ordering_rules.contains(&(y, x)) {
                Ordering::Greater
            } else {
                panic!("missing order between pages");
            }
        })
        .collect_vec()
}
