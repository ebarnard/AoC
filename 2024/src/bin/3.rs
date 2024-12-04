use nom::{
    bytes::complete::tag,
    combinator::complete,
    sequence::{delimited, separated_pair},
};

use aoc2024::nom::uint32;

const TEST_A: &str = include_str!("../../inputs/3a-test.txt");
const TEST_B: &str = include_str!("../../inputs/3b-test.txt");
const REAL: &str = include_str!("../../inputs/3-real.txt");

fn main() {
    assert_eq!(part1(TEST_A), 161);
    assert_eq!(part1(REAL), 184511516);
    assert_eq!(part2(TEST_B), 48);
    assert_eq!(part2(REAL), 90044227);
}

fn part1(mut input: &str) -> u32 {
    let mut mul = complete(delimited(
        tag("mul("),
        separated_pair(uint32, tag(","), uint32),
        tag(")"),
    ));

    let mut acc = 0;
    while !input.is_empty() {
        if let Ok((remainder, (a, b))) = mul(input) {
            acc += a * b;
            input = remainder;
        } else {
            input = &input[1..];
        }
    }

    acc
}

fn part2(mut input: &str) -> u32 {
    let mut mul = complete(delimited(
        tag("mul("),
        separated_pair(uint32, tag(","), uint32),
        tag(")"),
    ));
    let mut do_ = complete(tag::<_, _, ()>("do()"));
    let mut dont = complete(tag::<_, _, ()>("don't()"));

    let mut mul_enabled = true;
    let mut acc = 0;
    while !input.is_empty() {
        if let Ok((remainder, (a, b))) = mul(input) {
            if mul_enabled {
                acc += a * b;
            }
            input = remainder;
        } else if let Ok((remainder, _)) = do_(input) {
            mul_enabled = true;
            input = remainder;
        } else if let Ok((remainder, _)) = dont(input) {
            mul_enabled = false;
            input = remainder;
        } else {
            input = &input[1..];
        }
    }

    acc
}
