use std::{collections::HashSet, mem};

use itertools::Itertools;

const TEST: &str = include_str!("../../inputs/21-test.txt");
const REAL: &str = include_str!("../../inputs/21-real.txt");

struct StepsFromTile {
    steps_to_tile: Vec<Vec<Option<u32>>>,
    max_steps_to_tile: u32,
    total_even_tiles: u32,
    total_odd_tiles: u32,
}

fn main() {
    assert_eq!(part1(TEST, 6), 16);
    assert_eq!(part1(REAL, 64), 3847);
    assert_eq!(part2(REAL, 26501365), 637537341306357);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str, total_steps: u32) -> u32 {
    let map = parse(input);

    let (x0, y0) = find_start(&map);
    let steps_from_start = steps_from_tile(&map, x0, y0);

    steps_from_start.active_tiles(0, total_steps)
}

fn part2(input: &str, total_steps: u32) -> u64 {
    let map = parse(input);

    let width = map[0].len();
    let height = map.len();
    let (x0, y0) = find_start(&map);

    // Start is in the center of the map and has a clear line of sight to left,
    // right, up and down.
    assert_eq!(height, width);
    assert_eq!(x0, y0);
    assert_eq!(x0 * 2 + 1, width);
    assert!(map[y0].iter().all(|&t| t != '#'));
    assert!(map.iter().all(|row| row[x0] != '#'));

    let steps_from_start = steps_from_tile(&map, x0, y0);
    let steps_from_straights = [
        steps_from_tile(&map, x0, 0),
        steps_from_tile(&map, 0, y0),
        steps_from_tile(&map, x0, height - 1),
        steps_from_tile(&map, width - 1, y0),
    ];
    let steps_from_corners = [
        steps_from_tile(&map, 0, 0),
        steps_from_tile(&map, width - 1, 0),
        steps_from_tile(&map, 0, height - 1),
        steps_from_tile(&map, width - 1, height - 1),
    ];

    // Start map.
    let start_map_active = u64::from(steps_from_start.active_tiles(0, total_steps));

    // Maps straight up, left, down and right.
    let straight_maps_active: u64 = (1..)
        .map(|i| {
            let steps_to_reach_straights = (x0 + 1 + width * (i - 1)) as u32;
            steps_from_straights
                .iter()
                .map(|steps| u64::from(steps.active_tiles(steps_to_reach_straights, total_steps)))
                .sum::<u64>()
        })
        .take_while(|&active| active > 0)
        .sum();

    // Diagonal maps of increating value of |x + y|.
    let diagonal_maps_active: u64 = (2..)
        .map(|i| {
            let steps_to_reach_diagonals = (2 * (x0 + 1) + width * (i - 2)) as u32;
            steps_from_corners
                .iter()
                .map(|steps| u64::from(steps.active_tiles(steps_to_reach_diagonals, total_steps)))
                .sum::<u64>()
                * (i - 1) as u64
        })
        .take_while(|&active| active > 0)
        .sum();

    start_map_active + straight_maps_active + diagonal_maps_active
}

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    (0..map.len())
        .cartesian_product(0..map[0].len())
        .find(|&(x, y)| map[y][x] == 'S')
        .unwrap()
}

fn steps_from_tile(map: &Vec<Vec<char>>, x0: usize, y0: usize) -> StepsFromTile {
    let width = map[0].len();
    let height = map.len();

    let mut steps_to_tile = vec![vec![None; map[0].len()]; map.len()];
    let mut wavefront: HashSet<_> = [(x0, y0)].into_iter().collect();
    let mut next_wavefront = HashSet::new();
    for i in 0.. {
        for (x, y) in wavefront.drain() {
            steps_to_tile[y][x] = Some(i);

            let mut try_move = |x: usize, y: usize| {
                if map[y][x] != '#' && steps_to_tile[y][x] == None {
                    next_wavefront.insert((x, y));
                }
            };

            if x > 0 {
                try_move(x - 1, y);
            }
            if x < width - 1 {
                try_move(x + 1, y);
            }
            if y > 0 {
                try_move(x, y - 1);
            }
            if y < height - 1 {
                try_move(x, y + 1);
            }
        }

        if next_wavefront.is_empty() {
            break;
        }

        mem::swap(&mut wavefront, &mut next_wavefront);
    }

    StepsFromTile {
        max_steps_to_tile: steps_to_tile
            .iter()
            .flatten()
            .filter_map(|&s| s)
            .max()
            .unwrap(),
        total_even_tiles: calc_active_tiles(0, u32::MAX & 0xfffffffe, &steps_to_tile),
        total_odd_tiles: calc_active_tiles(0, u32::MAX | 1, &steps_to_tile),
        steps_to_tile,
    }
}

impl StepsFromTile {
    fn active_tiles(&self, steps_to_map: u32, total_steps: u32) -> u32 {
        if steps_to_map > total_steps {
            0
        } else if steps_to_map + self.max_steps_to_tile <= total_steps {
            if ((total_steps - steps_to_map) & 1) == 0 {
                self.total_even_tiles
            } else {
                self.total_odd_tiles
            }
        } else {
            calc_active_tiles(steps_to_map, total_steps, &self.steps_to_tile)
        }
    }
}

fn calc_active_tiles(
    steps_to_map: u32,
    total_steps: u32,
    min_steps_to_tile: &Vec<Vec<Option<u32>>>,
) -> u32 {
    min_steps_to_tile
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|&s| s)
        .filter(|&s| steps_to_map + s <= total_steps)
        .filter(|&s| (steps_to_map + s) & 1 == total_steps & 1)
        .count() as u32
}
