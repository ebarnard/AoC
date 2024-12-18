use std::{collections::HashSet, mem, u32};

use itertools::Itertools;
use nom::{bytes::complete::tag, multi::separated_list1, sequence::separated_pair};

use aoc2024::nom::{parse_all, uint32};

const TEST: &str = include_str!("../../inputs/18-test.txt");
const REAL: &str = include_str!("../../inputs/18-real.txt");

fn main() {
    assert_eq!(part1(TEST, 7, 12), 22);
    assert_eq!(part1(REAL, 71, 1024), 288);
    assert_eq!(part2(TEST, 7), (6, 1));
    assert_eq!(part2(REAL, 71), (52, 5));
}

fn parse(input: &str) -> Vec<(u32, u32)> {
    let position = separated_pair(uint32, tag(","), uint32);
    let positions = separated_list1(tag("\n"), position);
    parse_all(input, positions)
}

fn part1(input: &str, size: usize, take: usize) -> u32 {
    let corrupted_positions = parse(input);

    let mut distances = vec![vec![Some(u32::MAX); size]; size];
    distances[0][0] = Some(0);

    for (x, y) in corrupted_positions.into_iter().take(take) {
        distances[y as usize][x as usize] = None;
    }

    let mut wavefront: HashSet<_> = [(0, 0)].into_iter().collect();
    let mut next_wavefront = HashSet::new();
    while !wavefront.is_empty() {
        for (x, y) in wavefront.drain() {
            let d = distances[y][x].unwrap() + 1;

            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                let Some(nd) = distances
                    .get(ny as usize)
                    .and_then(|row| row.get(nx as usize))
                    .and_then(|d| *d)
                else {
                    continue;
                };

                if d < nd {
                    distances[ny][nx] = Some(d);
                    next_wavefront.insert((nx, ny));
                }
            }
        }

        mem::swap(&mut wavefront, &mut next_wavefront);
    }

    distances[size - 1][size - 1].unwrap()
}

fn part2(input: &str, size: usize) -> (u32, u32) {
    let corrupted_positions = parse(input);

    let counts = (1..=corrupted_positions.len()).collect_vec();
    let first_blocking_idx = counts.partition_point(|&take| part1(input, size, take) < u32::MAX);

    corrupted_positions[first_blocking_idx]
}
