use nom::{character::char, multi::separated_list0, sequence::separated_pair};

use aoc2025::nom::{parse_all, uint64};

const TEST: &str = include_str!("../../inputs/2-test.txt");
const REAL: &str = include_str!("../../inputs/2-real.txt");

fn main() {
    assert_eq!(part1(TEST), 1227775554);
    assert_eq!(part1(REAL), 23534117921);
    assert_eq!(part2(TEST), 4174379265);
    assert_eq!(part2(REAL), 31755323497);
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    let range = separated_pair(uint64, char('-'), uint64);
    let ranges = separated_list0(char(','), range);

    parse_all(input, ranges)
}

fn part1(input: &str) -> u64 {
    let ranges = parse(input);

    ranges
        .into_iter()
        .flat_map(|(start, end)| start..=end)
        .filter(|v| {
            let s = v.to_string();
            if s.len() % 2 == 0 {
                s[0..(s.len() / 2)] == s[(s.len() / 2)..]
            } else {
                false
            }
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let ranges = parse(input);

    ranges
        .into_iter()
        .flat_map(|(start, end)| start..=end)
        .filter(|v| {
            let s = v.to_string().into_bytes();
            (1..=(s.len() / 2)).any(|i| {
                let mut c = s.chunks_exact(i);
                if c.remainder().is_empty() {
                    c.all(|g| g == &s[..i])
                } else {
                    false
                }
            })
        })
        .sum()
}
