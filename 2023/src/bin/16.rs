use std::iter;

const TEST: &str = include_str!("../../inputs/16-test.txt");
const REAL: &str = include_str!("../../inputs/16-real.txt");

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum Dir {
    North,
    East,
    South,
    West,
}

fn main() {
    assert_eq!(part1(TEST), 46);
    assert_eq!(part1(REAL), 8539);
    assert_eq!(part2(TEST), 51);
    assert_eq!(part2(REAL), 8674);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> u32 {
    let map = parse(input);
    count_energised(&map, 0, 0, Dir::East)
}

fn part2(input: &str) -> u32 {
    let map = parse(input);
    let north = (0..map[0].len()).map(|i| (i, 0, Dir::South));
    let east = (0..map.len()).map(|i| (map.len() - 1, i, Dir::West));
    let south = (0..map[0].len()).map(|i| (i, map[0].len() - 1, Dir::North));
    let west = (0..map.len()).map(|i| (0, i, Dir::East));

    north
        .chain(east)
        .chain(south)
        .chain(west)
        .map(|(x, y, dir)| count_energised(&map, x, y, dir))
        .max()
        .unwrap_or(0)
}

fn count_energised(map: &Vec<Vec<char>>, x: usize, y: usize, dir: Dir) -> u32 {
    let mut energised: Vec<Vec<_>> = map
        .iter()
        .map(|row| row.iter().map(|_| 0).collect())
        .collect();

    follow_beam(&map, x, y, dir, &mut energised);

    energised
        .iter()
        .flat_map(|v| v.iter())
        .filter(|&&v| v != 0)
        .count() as u32
}

fn follow_beam(map: &Vec<Vec<char>>, x: usize, y: usize, dir: Dir, energised: &mut Vec<Vec<u8>>) {
    if energised[y][x] & (1 << dir as u8) != 0 {
        return;
    }
    energised[y][x] |= 1 << dir as u8;

    let (next_dir, split_dir) = interact(map[y][x], dir);

    for ndir in iter::once(next_dir).chain(split_dir) {
        if let Some((nx, ny)) = step(x, y, ndir, map[0].len(), map.len()) {
            follow_beam(map, nx, ny, ndir, energised);
        }
    }
}

fn interact(tile: char, dir: Dir) -> (Dir, Option<Dir>) {
    match (tile, dir) {
        ('.', _) => (dir, None),
        ('|', Dir::North | Dir::South) => (dir, None),
        ('|', Dir::East | Dir::West) => (Dir::North, Some(Dir::South)),
        ('-', Dir::North | Dir::South) => (Dir::East, Some(Dir::West)),
        ('-', Dir::East | Dir::West) => (dir, None),
        ('/', Dir::North) => (Dir::East, None),
        ('/', Dir::East) => (Dir::North, None),
        ('/', Dir::South) => (Dir::West, None),
        ('/', Dir::West) => (Dir::South, None),
        ('\\', Dir::North) => (Dir::West, None),
        ('\\', Dir::East) => (Dir::South, None),
        ('\\', Dir::South) => (Dir::East, None),
        ('\\', Dir::West) => (Dir::North, None),
        _ => panic!("unknown tile: {tile}"),
    }
}

fn step(x: usize, y: usize, dir: Dir, width: usize, height: usize) -> Option<(usize, usize)> {
    match dir {
        Dir::North if y > 0 => Some((x, y - 1)),
        Dir::East if x + 1 < width => Some((x + 1, y)),
        Dir::South if y + 1 < height => Some((x, y + 1)),
        Dir::West if x > 0 => Some((x - 1, y)),
        _ => None,
    }
}
