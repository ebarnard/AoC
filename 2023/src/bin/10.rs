use std::collections::HashSet;

use itertools::Itertools;

const TEST: &str = include_str!("../../inputs/10-test.txt");
const REAL: &str = include_str!("../../inputs/10-real.txt");

fn main() {
    assert_eq!(part1(TEST), 8);
    assert_eq!(part1(REAL), 6846);
    assert_eq!(part2(TEST), 1);
    assert_eq!(part2(REAL), 325);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> u32 {
    let map = parse(input);
    let loop_tiles = find_loop_tile_positions(&map);
    (loop_tiles.len() as u32 + 1) / 2
}

fn part2(input: &str) -> u32 {
    let map = parse(input);

    let loop_tiles: HashSet<(i32, i32)> = find_loop_tile_positions(&map).into_iter().collect();

    // Any line from any point inside a closed non-overlapping loop to a point
    // outside it must cross the loop an odd number of times, touching the loop
    // at a tangent does not count as a crossing.
    //
    // Iterate over each non-loop point and take a line at 45 deg towards the
    // origin. Count the number of times that line crosses the loop. Outer
    // corners ('L' and '7') are touched at a tangent and do not count as loop
    // crossings.
    //
    // Straight lines are harder as we would have to handle moving directly
    // along an edge.
    (0..map.len())
        .cartesian_product(0..map[0].len())
        .filter(|&(x, y)| !loop_tiles.contains(&(x as i32, y as i32)))
        .filter(|&(x, y)| {
            let crossings = (0..x).rev().zip((0..y).rev()).fold(0, |crossings, (x, y)| {
                (loop_tiles.contains(&(x as i32, y as i32)) && !matches!(map[y][x], 'L' | '7'))
                    as u32
                    + crossings
            });
            crossings % 2 == 1
        })
        .count() as u32
}

fn find_loop_tile_positions(map: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let (x0, y0) = find_start(&map);

    // Try each cardinal direction until we find one that is in a loop.
    [(0, -1), (-1, 0), (0, 1), (1, 0)]
        .into_iter()
        .find_map(|(dx, dy)| loop_tile_positions(&map, x0 as i32, y0 as i32, dx, dy))
        .unwrap()
}

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    (0..map.len())
        .cartesian_product(0..map[0].len())
        .find(|&(x, y)| map[y][x] == 'S')
        .unwrap()
}

fn loop_tile_positions(
    map: &Vec<Vec<char>>,
    x0: i32,
    y0: i32,
    mut dx: i32,
    mut dy: i32,
) -> Option<Vec<(i32, i32)>> {
    let (mut x, mut y) = (x0, y0);
    let mut positions = vec![(x, y)];
    loop {
        x += dx;
        y += dy;

        if x == x0 && y == y0 {
            return Some(positions);
        }

        if !(0..map[0].len() as i32).contains(&x) || !(0..map.len() as i32).contains(&y) {
            return None;
        }

        let Some((dir_a, dir_b)) = outgoing_directions(map[y as usize][x as usize]) else {
            return None;
        };

        positions.push((x, y));

        (dx, dy) = if dir_a == (-dx, -dy) {
            dir_b
        } else if dir_b == (-dx, -dy) {
            dir_a
        } else {
            panic!();
        };
    }
}

fn outgoing_directions(tile: char) -> Option<((i32, i32), (i32, i32))> {
    match tile {
        '|' => Some(((0, -1), (0, 1))),
        '-' => Some(((-1, 0), (1, 0))),
        'L' => Some(((0, -1), (1, 0))),
        'J' => Some(((0, -1), (-1, 0))),
        '7' => Some(((-1, 0), (0, 1))),
        'F' => Some(((0, 1), (1, 0))),
        '.' => None,
        _ => panic!("unknown tile {tile}"),
    }
}
