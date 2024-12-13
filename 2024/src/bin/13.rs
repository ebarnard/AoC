use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
};

use aoc2024::nom::{parse_all, uint64};

const TEST: &str = include_str!("../../inputs/13-test.txt");
const REAL: &str = include_str!("../../inputs/13-real.txt");

fn main() {
    assert_eq!(part1(TEST), 480);
    assert_eq!(part1(REAL), 28887);
    assert_eq!(part2(REAL), 96979582619758);
}

fn parse(input: &str) -> Vec<((u64, u64), (u64, u64), (u64, u64))> {
    let num = |prefix| preceded(tag(prefix), uint64);

    let line = |start, x_prefix, y_prefix| {
        delimited(
            pair(tag(start), tag(": ")),
            separated_pair(num(x_prefix), tag(", "), num(y_prefix)),
            tag("\n"),
        )
    };

    let machine = tuple((
        line("Button A", "X+", "Y+"),
        line("Button B", "X+", "Y+"),
        line("Prize", "X=", "Y="),
    ));

    let machines = separated_list1(tag("\n"), machine);
    parse_all(input, machines)
}

fn part1(input: &str) -> u64 {
    let machines = parse(input);
    total_tokens(&machines)
}

fn part2(input: &str) -> u64 {
    let machines = parse(input);

    let machines = machines
        .into_iter()
        .map(|(a, b, (px, py))| (a, b, (px + 10000000000000, py + 10000000000000)))
        .collect_vec();

    total_tokens(&machines)
}

fn total_tokens(machines: &[((u64, u64), (u64, u64), (u64, u64))]) -> u64 {
    machines
        .iter()
        .map(|&((ax, ay), (bx, by), (px, py))| {
            let det = (ax * by) as i64 - (bx * ay) as i64;
            assert!(det != 0);

            let a = (by * px) as i64 - (bx * py) as i64;
            let b = (ax * py) as i64 - (ay * px) as i64;

            if a % det == 0 && b % det == 0 {
                let a = a / det;
                let b = b / det;

                (a * 3 + b * 1) as u64
            } else {
                0
            }
        })
        .sum()
}
