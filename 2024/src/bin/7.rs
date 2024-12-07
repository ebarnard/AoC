use nom::{bytes::complete::tag, multi::separated_list1, sequence::separated_pair};

use aoc2024::nom::{parse_all, uint64, ws};

const TEST: &str = include_str!("../../inputs/7-test.txt");
const REAL: &str = include_str!("../../inputs/7-real.txt");

fn main() {
    assert_eq!(part1(TEST), 3749);
    assert_eq!(part1(REAL), 20281182715321);
    assert_eq!(part2(TEST), 11387);
    assert_eq!(part2(REAL), 159490400628354);
}

fn parse(input: &str) -> Vec<(u64, Vec<u64>)> {
    let equation = separated_pair(uint64, tag(": "), separated_list1(ws, uint64));
    let equations = separated_list1(tag("\n"), equation);
    parse_all(input, equations)
}

fn part1(input: &str) -> u64 {
    total_calibration_result(input, &[add, mul])
}

fn part2(input: &str) -> u64 {
    total_calibration_result(input, &[add, mul, concat])
}

fn total_calibration_result(input: &str, ops: &[fn(u64, u64) -> u64]) -> u64 {
    let equations = parse(input);

    equations
        .into_iter()
        .filter(|(target, inputs)| can_produce_target(inputs, *target, ops))
        .map(|(target, _)| target)
        .sum()
}

fn can_produce_target(inputs: &[u64], target: u64, ops: &[fn(u64, u64) -> u64]) -> bool {
    fn recurse(inputs: &[u64], acc: u64, target: u64, ops: &[fn(u64, u64) -> u64]) -> bool {
        if inputs.is_empty() {
            acc == target
        } else {
            let (&value, inputs) = inputs.split_first().unwrap();
            ops.into_iter()
                .any(|op| recurse(inputs, op(acc, value), target, ops))
        }
    }

    let (&value, inputs) = inputs.split_first().unwrap();
    recurse(inputs, value, target, ops)
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn mul(a: u64, b: u64) -> u64 {
    a * b
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}
