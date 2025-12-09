const TEST: &str = include_str!("../../inputs/7-test.txt");
const REAL: &str = include_str!("../../inputs/7-real.txt");

fn main() {
    assert_eq!(part1(TEST), 21);
    assert_eq!(part1(REAL), 1592);
    assert_eq!(part2(TEST), 40);
    assert_eq!(part2(REAL), 17921968177009);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(input: &str) -> u32 {
    let map = parse(input);
    beam_splits_and_counts(&map).0
}

fn part2(input: &str) -> u64 {
    let map = parse(input);
    beam_splits_and_counts(&map).1
}

fn beam_splits_and_counts(map: &Vec<Vec<char>>) -> (u32, u64) {
    let num_rows = map.len();
    let num_cols = map[0].len();

    let mut beam_splits = 0;
    let mut beam_counts = vec![vec![0; num_cols]; num_rows];
    for y in 0..(num_rows - 1) {
        for x in 0..num_cols {
            let beam_count = match map[y][x] {
                'S' => 1,
                _ => beam_counts[y][x],
            };

            if beam_count == 0 {
                continue;
            }

            if map[y + 1][x] == '^' {
                beam_splits += 1;
                beam_counts[y + 1][x - 1] += beam_count;
                beam_counts[y + 1][x + 1] += beam_count;
            } else {
                beam_counts[y + 1][x] += beam_count;
            }
        }
    }

    (beam_splits, beam_counts.last().unwrap().iter().sum())
}
