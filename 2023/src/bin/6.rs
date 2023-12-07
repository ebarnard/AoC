use aoc2023::nom::{uint64, ws};
use nom::{
    bytes::complete::tag,
    combinator::{complete, map},
    multi::many1,
    sequence::{preceded, separated_pair},
    IResult,
};

const TEST: &str = include_str!("../../inputs/6-test.txt");
const REAL: &str = include_str!("../../inputs/6-real.txt");

#[derive(Debug)]
struct Race {
    time: u64,
    distance_record: u64,
}

fn main() {
    assert_eq!(part1(TEST), 288);
    assert_eq!(part1(REAL), 505494);
    assert_eq!(part2(TEST), 71503);
    assert_eq!(part2(REAL), 23632299);
}

fn parse(s: &str) -> Vec<Race> {
    fn races(s: &str) -> IResult<&str, Vec<Race>> {
        map(
            separated_pair(
                preceded(tag("Time:"), many1(ws(uint64))),
                tag("\n"),
                preceded(tag("Distance:"), many1(ws(uint64))),
            ),
            |(times, distances)| {
                times
                    .into_iter()
                    .zip(distances)
                    .map(|(time, distance_record)| Race {
                        time,
                        distance_record,
                    })
                    .collect()
            },
        )(s)
    }

    let (_, races) = complete(races)(s).unwrap();
    races
}

fn part1(input: &str) -> u64 {
    let races = parse(input);

    races
        .into_iter()
        .map(|race| {
            (0..=race.time)
                .filter(|hold_time| hold_time * (race.time - hold_time) > race.distance_record)
                .count()
        })
        .product::<usize>() as u64
}

fn part2(input: &str) -> u64 {
    let races = parse(&input.replace(" ", ""));
    assert_eq!(races.len(), 1);

    // Solve quadratic `x * (race.time - x) = race.distance_record`.
    // Solution is number of non-negative integers between the two `x` values.
    let t = races[0].time as f64;
    let r = races[0].distance_record as f64;
    let x0 = (t - (t * t - 4.0 * r).sqrt()) / 2.0;
    let x1 = (t + (t * t - 4.0 * r).sqrt()) / 2.0;

    ((1.0 + x1).floor() - x0.ceil()) as u64
}
