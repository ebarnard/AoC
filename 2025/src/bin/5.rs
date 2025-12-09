use std::{cmp, collections::BTreeMap};

use nom::{bytes::tag, multi::separated_list0, sequence::separated_pair};

use aoc2025::nom::{parse_all, uint64};

const TEST: &str = include_str!("../../inputs/5-test.txt");
const REAL: &str = include_str!("../../inputs/5-real.txt");

fn main() {
    assert_eq!(part1(TEST), 3);
    assert_eq!(part1(REAL), 640);
    assert_eq!(part2(TEST), 14);
    assert_eq!(part2(REAL), 365804144481581);
}

fn parse(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let range = separated_pair(uint64, tag("-"), uint64);
    let ranges = separated_list0(tag("\n"), range);

    let values = separated_list0(tag("\n"), uint64);

    let parser = separated_pair(ranges, tag("\n\n"), values);

    parse_all(input, parser)
}

fn part1(input: &str) -> u32 {
    let (ranges, values) = parse(input);

    values
        .iter()
        .filter(|&v| ranges.iter().any(|&(a, b)| (a..=b).contains(v)))
        .count() as u32
}

fn part2(input: &str) -> u64 {
    let (ranges, _) = parse(input);

    let mut r = BTreeMap::<u64, u64>::new();

    for (start, end) in ranges {
        // Invariant: all inserted ranges are disjoint.
        // Find overlapping ranges (there are 0, 1 or 2), merged range is min(starts)..=max(ends).
        let overlapping = r.extract_if(.., |&s, &mut e| overlaps(start, end, s, e));

        let (merged_s, merge_e) = overlapping.fold((start, end), |(u, v), (s, e)| {
            (cmp::min(u, s), cmp::max(v, e))
        });

        r.insert(merged_s, merge_e);
    }

    r.iter().map(|(&s, &e)| e - s + 1).sum()
}

fn overlaps(a: u64, b: u64, u: u64, v: u64) -> bool {
    v >= a && u <= b
}
