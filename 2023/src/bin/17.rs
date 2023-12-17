use std::mem;

const TEST: &str = include_str!("../../inputs/17-test.txt");
const REAL: &str = include_str!("../../inputs/17-real.txt");

const MAX_STEPS: usize = 10;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
enum Dir {
    Up,
    Left,
    Down,
    Right,
}

fn main() {
    assert_eq!(part1(TEST), 102);
    assert_eq!(part1(REAL), 1044);
    assert_eq!(part2(TEST), 94);
    assert_eq!(part2(REAL), 1227);
}

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part1(input: &str) -> u32 {
    let map = parse(input);
    min_distance(&map, 0, 3)
}

fn part2(input: &str) -> u32 {
    let map = parse(input);
    min_distance(&map, 3, 10)
}

fn min_distance(
    map: &Vec<Vec<u32>>,
    min_steps_before_turn: usize,
    max_steps_in_line: usize,
) -> u32 {
    let mut min_distances: Vec<Vec<_>> = map
        .iter()
        .map(|row| row.iter().map(|_| [[u32::MAX; MAX_STEPS]; 4]).collect())
        .collect();
    min_distances[0][0] = [[0; MAX_STEPS]; 4];

    let mut wavefront = vec![(0, 0, Dir::Right, 0), (0, 0, Dir::Down, 0)];
    let mut next_wavefront = Vec::new();
    while !wavefront.is_empty() {
        for (x, y, dir, step) in wavefront.drain(..) {
            let min_distance = min_distances[y][x][dir as usize][step];
            assert_ne!(min_distance, u32::MAX);

            // Continue in the same direction or turn left or right as allowed.
            let actions = match dir {
                Dir::Up | Dir::Down => [(Dir::Left, 0), (Dir::Right, 0)],
                Dir::Left | Dir::Right => [(Dir::Up, 0), (Dir::Down, 0)],
            }
            .into_iter()
            .filter(|_| step >= min_steps_before_turn)
            .chain(Some((dir, step + 1)).filter(|&(_, s)| s < max_steps_in_line));

            for (ndir, nstep) in actions {
                let Some((nx, ny)) = next_tile(map, x, y, ndir) else {
                    continue;
                };

                let dist = min_distance + map[ny][nx];
                if dist < min_distances[ny][nx][ndir as usize][nstep] {
                    min_distances[ny][nx][ndir as usize][nstep] = dist;
                    next_wavefront.push((nx, ny, ndir, nstep));
                }
            }
        }
        mem::swap(&mut wavefront, &mut next_wavefront);
    }

    *min_distances
        .last()
        .and_then(|row| row.last())
        .into_iter()
        .flatten()
        .flat_map(|steps| &steps[min_steps_before_turn..max_steps_in_line])
        .min()
        .unwrap()
}

fn next_tile(map: &Vec<Vec<u32>>, x: usize, y: usize, dir: Dir) -> Option<(usize, usize)> {
    match dir {
        Dir::Up if y > 0 => Some((x, y - 1)),
        Dir::Left if x > 0 => Some((x - 1, y)),
        Dir::Down if y + 1 < map.len() => Some((x, y + 1)),
        Dir::Right if x + 1 < map[0].len() => Some((x + 1, y)),
        _ => None,
    }
}
