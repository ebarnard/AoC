use std::collections::HashSet;

use itertools::Itertools;

const TEST: &str = include_str!("../../inputs/12-test.txt");
const REAL: &str = include_str!("../../inputs/12-real.txt");

fn main() {
    assert_eq!(part1(TEST), 1930);
    assert_eq!(part1(REAL), 1465968);
    assert_eq!(part2(TEST), 1206);
    assert_eq!(part2(REAL), 897702);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> u64 {
    let a = [
        [None, Some(false), None],
        [None, Some(true), None],
        [None, None, None],
    ];
    let b = filter_rotate_90(a);
    let c = filter_rotate_90(b);
    let d = filter_rotate_90(c);
    let filters = [a, b, c, d];

    total_cost(input, &filters)
}

fn part2(input: &str) -> u64 {
    let a = [
        [None, Some(false), None],
        [Some(false), Some(true), None],
        [None, None, None],
    ];
    let b = filter_rotate_90(a);
    let c = filter_rotate_90(b);
    let d = filter_rotate_90(c);
    let e = [
        [Some(true), Some(false), None],
        [Some(true), Some(true), None],
        [None, None, None],
    ];
    let f = filter_rotate_90(e);
    let g = filter_rotate_90(f);
    let h = filter_rotate_90(g);

    let filters = [a, b, c, d, e, f, g, h];

    total_cost(input, &filters)
}

fn total_cost(input: &str, filters: &[[[Option<bool>; 3]; 3]]) -> u64 {
    let map = parse(input);
    let (w, h) = (map[0].len(), map.len());

    let mut visited = vec![vec![false; w]; h];
    (0..w)
        .cartesian_product(0..h)
        .filter_map(|(x, y)| {
            if visited[y][x] {
                None
            } else {
                Some(region_cost(&map, x, y, &mut visited, &filters))
            }
        })
        .sum()
}

fn region_cost(
    map: &[Vec<char>],
    x0: usize,
    y0: usize,
    visited: &mut [Vec<bool>],
    filters: &[[[Option<bool>; 3]; 3]],
) -> u64 {
    let region = map[y0][x0];
    let mut wavefront: HashSet<_> = [(x0, y0)].into_iter().collect();
    let mut area = 0;
    let mut matches = 0;
    while !wavefront.is_empty() {
        let mut next_wavefront = HashSet::new();
        for (x, y) in wavefront.into_iter() {
            assert_eq!(map[y][x], region);
            assert!(!visited[y][x]);

            visited[y][x] = true;
            area += 1;
            matches += filters
                .iter()
                .filter(|f| filter_matches(&map, x, y, f))
                .count() as u32;

            let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
            for (dx, dy) in directions {
                if let Some((u, v, r)) = get_region(&map, x, y, dx, dy) {
                    if r == region && !visited[v][u] {
                        next_wavefront.insert((u, v));
                    }
                }
            }
        }
        wavefront = next_wavefront;
    }

    area as u64 * matches as u64
}

fn filter_rotate_90(input: [[Option<bool>; 3]; 3]) -> [[Option<bool>; 3]; 3] {
    let w = input[0].len();
    let h = input.len();

    let mut output = [[None; 3]; 3];
    for x in 0..w {
        for y in 0..h {
            output[x][h - 1 - y] = input[y][x];
        }
    }

    output
}

fn filter_matches(
    map: &[Vec<char>],
    x0: usize,
    y0: usize,
    pattern: &[[Option<bool>; 3]; 3],
) -> bool {
    let region = map[y0][x0];
    (-1..=1).cartesian_product(-1..=1).all(|(dx, dy)| {
        let r = get_region(map, x0, y0, dx, dy).map(|(_, _, r)| r);

        match (
            pattern[(dy + 1) as usize][(dx + 1) as usize],
            r == Some(region),
        ) {
            (Some(true), false) => false,
            (Some(false), true) => false,
            _ => true,
        }
    })
}

fn get_region(
    map: &[Vec<char>],
    x0: usize,
    y0: usize,
    dx: isize,
    dy: isize,
) -> Option<(usize, usize, char)> {
    let (x, y) = (x0.wrapping_add_signed(dx), y0.wrapping_add_signed(dy));
    Some((x, y, *map.get(y)?.get(x)?))
}
