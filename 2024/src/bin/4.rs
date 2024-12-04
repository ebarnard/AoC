use itertools::Itertools;

const TEST: &str = include_str!("../../inputs/4-test.txt");
const REAL: &str = include_str!("../../inputs/4-real.txt");

fn main() {
    assert_eq!(part1(TEST), 18);
    assert_eq!(part1(REAL), 2642);
    assert_eq!(part2(TEST), 9);
    assert_eq!(part2(REAL), 1974);
}

fn part1(input: &str) -> u32 {
    let pattern_a = vec![vec!['X', 'M', 'A', 'S']];

    let pattern_b = vec![
        vec!['X', '-', '-', '-'],
        vec!['-', 'M', '-', '-'],
        vec!['-', '-', 'A', '-'],
        vec!['-', '-', '-', 'S'],
    ];

    rotated_pattern_matches(input, &pattern_a) + rotated_pattern_matches(input, &pattern_b)
}

fn part2(input: &str) -> u32 {
    let pattern = vec![
        vec!['M', '-', 'M'],
        vec!['-', 'A', '-'],
        vec!['S', '-', 'S'],
    ];

    rotated_pattern_matches(input, &pattern)
}

fn rotated_pattern_matches(input: &str, pattern: &Vec<Vec<char>>) -> u32 {
    let input = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let pattern_90 = pattern_rotate_90(&pattern);
    let pattern_180 = pattern_rotate_90(&pattern_90);
    let pattern_270 = pattern_rotate_90(&pattern_180);

    [pattern, &pattern_90, &pattern_180, &pattern_270]
        .into_iter()
        .map(|filter| single_pattern_matches(&input, filter))
        .sum()
}

fn pattern_rotate_90(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let w = input[0].len();
    let h = input.len();

    let mut output = vec![vec!['-'; h]; w];
    for x in 0..w {
        for y in 0..h {
            output[x][h - 1 - y] = input[y][x];
        }
    }

    output
}

fn single_pattern_matches(input: &Vec<Vec<char>>, pattern: &Vec<Vec<char>>) -> u32 {
    let w = input[0].len();
    let h = input.len();
    let pw = pattern[0].len();
    let ph = pattern.len();

    (0..=(w - pw))
        .cartesian_product(0..=(h - ph))
        .filter(|&(x, y)| {
            (0..pw)
                .cartesian_product(0..ph)
                .all(|(u, v)| pattern[v][u] == '-' || input[y + v][x + u] == pattern[v][u])
        })
        .count() as u32
}
