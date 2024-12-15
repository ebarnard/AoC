use std::collections::HashMap;

use itertools::Itertools;

const TEST: &str = include_str!("../../inputs/10-test.txt");
const REAL: &str = include_str!("../../inputs/10-real.txt");

fn main() {
    assert_eq!(part1(TEST), 36);
    assert_eq!(part1(REAL), 646);
    assert_eq!(part2(TEST), 81);
    assert_eq!(part2(REAL), 1494);
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    let mut dst = [0; 4];
                    let dst = c.encode_utf8(&mut dst);
                    dst.parse().unwrap()
                })
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> u32 {
    let map = parse(input);

    let mut trail_count = HashMap::new();
    trailheads(&map)
        .map(|(x, y)| {
            trail_count.clear();
            count_trails(&map, x, y, &mut trail_count);
            trail_count.len() as u32
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let map = parse(input);

    let mut trail_count = HashMap::new();
    for (x0, y0) in trailheads(&map) {
        count_trails(&map, x0, y0, &mut trail_count);
    }

    trail_count.values().sum()
}

fn trailheads<'a>(map: &'a [Vec<u8>]) -> impl Iterator<Item = (usize, usize)> + 'a {
    (0..map[0].len())
        .cartesian_product(0..map.len())
        .filter(|&(x, y)| map[y][x] == 0)
}

fn count_trails(
    map: &[Vec<u8>],
    x: usize,
    y: usize,
    trail_count: &mut HashMap<(usize, usize), u32>,
) {
    if map[y][x] == 9 {
        *trail_count.entry((y, x)).or_insert(0) += 1;
    } else {
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if let Some((nx, ny)) = can_move(&map, x, y, dx, dy) {
                count_trails(map, nx, ny, trail_count);
            }
        }
    }
}

fn can_move(map: &[Vec<u8>], x: usize, y: usize, dx: isize, dy: isize) -> Option<(usize, usize)> {
    let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));

    if map[y][x] + 1 == *map.get(ny)?.get(nx)? {
        Some((nx, ny))
    } else {
        None
    }
}
