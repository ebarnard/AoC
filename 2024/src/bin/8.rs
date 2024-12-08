use std::collections::HashMap;

use itertools::Itertools;

const TEST: &str = include_str!("../../inputs/8-test.txt");
const REAL: &str = include_str!("../../inputs/8-real.txt");

fn main() {
    assert_eq!(part1(TEST), 14);
    assert_eq!(part1(REAL), 323);
    assert_eq!(part2(TEST), 34);
    assert_eq!(part2(REAL), 1077);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> u32 {
    let map = parse(input);
    let antennas = antennas_by_frequency(&map);

    antennas
        .values()
        .flat_map(|a| a.iter().tuple_combinations())
        .flat_map(|(&(x1, y1), &(x2, y2))| {
            let (dx, dy) = (x2 as i32 - x1 as i32, y2 as i32 - y1 as i32);
            [step(x1, y1, dx, dy, -1), step(x2, y2, dx, dy, 1)]
        })
        .filter(|&(x, y)| in_bounds(&map, x, y))
        .unique()
        .count() as u32
}

fn part2(input: &str) -> u32 {
    let map = parse(input);
    let antennas = antennas_by_frequency(&map);

    antennas
        .values()
        .flat_map(|a| a.iter().tuple_combinations())
        .flat_map(|(&(x1, y1), &(x2, y2))| {
            let (dx, dy) = (x2 as i32 - x1 as i32, y2 as i32 - y1 as i32);
            [(x1, y1, dx, dy, 1), (x1, y1, dx, dy, -1)]
        })
        .flat_map(|(x, y, dx, dy, s)| {
            (0..)
                .map(move |i| step(x, y, dx, dy, s * i))
                .take_while(|&(x, y)| in_bounds(&map, x, y))
        })
        .unique()
        .count() as u32
}

fn antennas_by_frequency(map: &[Vec<char>]) -> HashMap<char, Vec<(u32, u32)>> {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, &v)| (v, (x as u32, y as u32)))
        })
        .filter(|&(v, _)| v != '.')
        .into_group_map()
}

fn step(x: u32, y: u32, dx: i32, dy: i32, n: i32) -> (u32, u32) {
    let u = x.wrapping_add_signed(n * dx);
    let v = y.wrapping_add_signed(n * dy);
    (u, v)
}

fn in_bounds(map: &[Vec<char>], x: u32, y: u32) -> bool {
    let (w, h) = (map[0].len() as u32, map.len() as u32);
    x < w && y < h
}
