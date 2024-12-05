use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list0};

use aoc2024::nom::{parse_all, uint32, ws};

const TEST: &str = include_str!("../../inputs/2-test.txt");
const REAL: &str = include_str!("../../inputs/2-real.txt");

fn main() {
    assert_eq!(part1(TEST), 2);
    assert_eq!(part1(REAL), 490);
    assert_eq!(part2(TEST), 4);
    assert_eq!(part2(REAL), 536);
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    let line = separated_list0(ws, uint32);
    let lines = separated_list0(tag("\n"), line);

    parse_all(input, lines)
}

fn is_safe(report: impl Iterator<Item = u32> + Clone) -> bool {
    let mut pairs = report.tuple_windows();

    let ascending = pairs.clone().all(|(a, b)| a < b);
    let descending = pairs.clone().all(|(a, b)| a > b);
    let difference_in_bounds = pairs.all(|(a, b)| (1..=3).contains(&a.abs_diff(b)));

    (ascending || descending) && difference_in_bounds
}

fn part1(input: &str) -> u32 {
    let reports = parse(input);

    reports
        .into_iter()
        .filter(|report| is_safe(report.iter().copied()))
        .count() as u32
}

fn part2(input: &str) -> u32 {
    let reports = parse(input);

    reports
        .into_iter()
        .filter(|report| {
            (0..report.len()).any(|i| {
                let report_without_i = (&report[0..i]).iter().chain(&report[i + 1..]).copied();
                is_safe(report_without_i)
            })
        })
        .count() as u32
}
