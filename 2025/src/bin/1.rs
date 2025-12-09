use nom::{bytes::complete::tag, character::anychar, multi::separated_list0, sequence::pair};

use aoc2025::nom::{int32, parse_all};

const TEST: &str = include_str!("../../inputs/1-test.txt");
const REAL: &str = include_str!("../../inputs/1-real.txt");

fn main() {
    assert_eq!(part1(TEST), 3);
    assert_eq!(part1(REAL), 964);
    assert_eq!(part2(TEST), 6);
    assert_eq!(part2(REAL), 5872);
}

fn parse(input: &str) -> Vec<(char, i32)> {
    let line = pair(anychar, int32);
    let lines = separated_list0(tag("\n"), line);

    parse_all(input, lines)
}

fn part1(input: &str) -> u32 {
    let instructions = parse(input);

    let mut pos = 50;
    let mut count = 0;
    for (direction, distance) in instructions {
        match direction {
            'L' => pos -= distance,
            'R' => pos += distance,
            _ => unreachable!(),
        }

        pos = pos % 100;
        if pos < 0 {
            pos += 100;
        }

        if pos == 0 {
            count += 1;
        }
    }

    count
}

fn part2(input: &str) -> u32 {
    let instructions = parse(input);

    let mut pos = 50;
    let mut count = 0;
    for (direction, distance) in instructions {
        let delta = match direction {
            'L' => -distance,
            'R' => distance,
            _ => unreachable!(),
        };

        let (new_pos, zeros) = part2_count_zeros(pos, delta);

        println!("{direction}{distance} {new_pos} {zeros}");

        pos = new_pos;
        count += zeros;
    }

    count
}

fn part2_count_zeros(pos: i32, delta: i32) -> (i32, u32) {
    assert_ne!(delta, 0);

    let new_pos = pos + delta;

    // Did we land on exactly zero
    if new_pos == 0 {
        return (0, 1);
    }

    let mut zeros = (new_pos / 100).abs() as u32;

    // Add a zero for sign change
    if pos * new_pos < 0 {
        zeros += 1;
    }

    let mut new_pos_mod = new_pos % 100;
    if new_pos_mod < 0 {
        new_pos_mod += 100;
    }

    (new_pos_mod, zeros)
}

#[test]
fn test() {
    assert_eq!(-200 / 100, -2);
    assert_eq!(-200 % 100, 0);

    assert_eq!(-100 / 100, -1);
    assert_eq!(-100 % 100, 0);

    assert_eq!(-101 / 100, -1);
    assert_eq!(-101 % 100, -1);

    assert_eq!(part2_count_zeros(0, 20), (20, 0));
    assert_eq!(part2_count_zeros(0, 100), (0, 1));
    assert_eq!(part2_count_zeros(0, 101), (1, 1));

    assert_eq!(part2_count_zeros(0, -20), (80, 0));
    assert_eq!(part2_count_zeros(0, -100), (0, 1));
    assert_eq!(part2_count_zeros(0, -101), (99, 1));
    assert_eq!(part2_count_zeros(0, -200), (0, 2));
    assert_eq!(part2_count_zeros(0, -201), (99, 2));

    assert_eq!(part2_count_zeros(1, -20), (81, 1));
    assert_eq!(part2_count_zeros(1, -100), (1, 1));
    assert_eq!(part2_count_zeros(1, -101), (0, 2));
    assert_eq!(part2_count_zeros(1, -200), (1, 2));
    assert_eq!(part2_count_zeros(1, -201), (0, 3));
}
