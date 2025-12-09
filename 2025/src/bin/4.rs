use std::u32;

use itertools::Itertools;

const TEST: &str = include_str!("../../inputs/4-test.txt");
const REAL: &str = include_str!("../../inputs/4-real.txt");

fn main() {
    assert_eq!(part1(TEST), 13);
    assert_eq!(part1(REAL), 1437);
    assert_eq!(part2(TEST), 43);
    assert_eq!(part2(REAL), 8765);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> u32 {
    let map = parse(input);

    (0..map[0].len())
        .cartesian_product(0..map.len())
        .filter(|&(x, y)| is_paper_and_removable(&map, x, y))
        .count() as u32
}

fn part2(input: &str) -> u32 {
    let mut map = parse(input);

    let mut removed = 0;
    let mut prev_removed = u32::MAX;
    while removed != prev_removed {
        prev_removed = removed;

        for (x, y) in (0..map[0].len()).cartesian_product(0..map.len()) {
            if is_paper_and_removable(&map, x, y) {
                map[y][x] = '.';
                removed += 1;
            }
        }
    }
    removed
}

fn is_paper_and_removable(map: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    is_paper(&map, x, y, 0, 0)
        && [-1, 0, 1]
            .into_iter()
            .cartesian_product([-1, 0, 1])
            .filter(|&(dx, dy)| !(dx == 0 && dy == 0))
            .filter(|&(dx, dy)| is_paper(&map, x, y, dx, dy))
            .count()
            < 4
}

fn is_paper(map: &Vec<Vec<char>>, x: usize, y: usize, dx: isize, dy: isize) -> bool {
    map.get(y.wrapping_add_signed(dy))
        .and_then(|row: &Vec<char>| row.get(x.wrapping_add_signed(dx)))
        .map(|&c| c == '@')
        .unwrap_or(false)
}
