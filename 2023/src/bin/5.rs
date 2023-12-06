use std::{cmp, ops::Range};

use aoc2023::nom::{uint64, ws};
use nom::{
    bytes::complete::{tag, take_while1},
    combinator::{complete, map},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

const TEST: &str = include_str!("../../inputs/5-test.txt");
const REAL: &str = include_str!("../../inputs/5-real.txt");

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    name: String,
    mappings: Vec<Mapping>,
}

#[derive(Debug)]
struct Mapping {
    source: Range<u64>,
    dest: Range<u64>,
}

fn main() {
    assert_eq!(part1(TEST), 35);
    assert_eq!(part1(REAL), 175622908);
    assert_eq!(part2(TEST), 46);
    assert_eq!(part2(REAL), 5200543);
}

fn parse(s: &str) -> Almanac {
    fn almanac(s: &str) -> IResult<&str, Almanac> {
        map(
            pair(
                delimited(tag("seeds:"), many1(ws(uint64)), tag("\n\n")),
                separated_list1(tag("\n\n"), mappings),
            ),
            |(seeds, maps)| Almanac { seeds, maps },
        )(s)
    }

    fn mappings(s: &str) -> IResult<&str, Map> {
        map(
            pair(
                terminated(take_while1(|c| c != ' '), tag(" map:\n")),
                separated_list1(tag("\n"), mapping),
            ),
            |(name, mappings)| Map {
                name: name.to_owned(),
                mappings,
            },
        )(s)
    }

    fn mapping(s: &str) -> IResult<&str, Mapping> {
        map(
            tuple((ws(uint64), ws(uint64), ws(uint64))),
            |(dest_start, source_start, len)| Mapping {
                source: source_start..(source_start + len),
                dest: dest_start..(dest_start + len),
            },
        )(s)
    }

    let (_, almanac) = complete(almanac)(s).unwrap();
    almanac
}

fn part1(input: &str) -> u64 {
    let almanac = parse(input);

    almanac
        .seeds
        .iter()
        .map(|&seed| {
            almanac.maps.iter().fold(seed, |value, map| {
                map.mappings
                    .iter()
                    .filter(|mapping| mapping.source.contains(&value))
                    .map(|mapping| value - mapping.source.start + mapping.dest.start)
                    .next()
                    .unwrap_or(value)
            })
        })
        .min()
        .unwrap()
}

fn part2(input: &str) -> u64 {
    let almanac = parse(input);

    let seed_ranges: Vec<Range<u64>> = almanac
        .seeds
        .chunks_exact(2)
        .map(|s| s[0]..(s[0] + s[1]))
        .collect();

    almanac
        .maps
        .iter()
        .fold(seed_ranges, |ranges, map| {
            ranges
                .into_iter()
                .flat_map(|range| {
                    let mut mapped_ranges = vec![];
                    let mut remaining = range.clone();
                    while !remaining.is_empty() {
                        let r = remaining.clone();

                        let (start, len) = if let Some(m) = map
                            .mappings
                            .iter()
                            .filter(|m| m.source.contains(&r.start))
                            .next()
                        {
                            let start = m.dest.start + (r.start - m.source.start);
                            let len = cmp::min(r.end, m.source.end) - r.start;
                            (start, len)
                        } else if let Some(m) = map
                            .mappings
                            .iter()
                            .filter(|m| r.contains(&m.source.start))
                            .next()
                        {
                            (r.start, m.source.start - r.start)
                        } else {
                            (r.start, r.end - r.start)
                        };

                        remaining = (r.start + len)..r.end;
                        mapped_ranges.push(start..(start + len));
                    }
                    println!(
                        "{}: range {:?} maps to {:?}",
                        map.name, range, mapped_ranges
                    );
                    mapped_ranges
                })
                .collect()
        })
        .into_iter()
        .map(|r| r.start)
        .min()
        .unwrap()
}
