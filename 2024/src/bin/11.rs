use std::collections::HashMap;

const TEST: &str = include_str!("../../inputs/11-test.txt");
const REAL: &str = include_str!("../../inputs/11-real.txt");

fn main() {
    assert_eq!(part1(TEST), 55312);
    assert_eq!(part1(REAL), 183248);
    assert_eq!(part2(REAL), 218811774248729);
}

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(input: &str) -> u64 {
    count_stones(input, 25)
}

fn part2(input: &str) -> u64 {
    count_stones(input, 75)
}

fn count_stones(input: &str, blinks: u32) -> u64 {
    let stones = parse(input);

    let mut memo = HashMap::new();
    stones
        .iter()
        .map(|&v| count_stones_rec(v, blinks, &mut memo))
        .sum()
}

fn count_stones_rec(value: u64, blinks: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }

    if let Some(&count) = memo.get(&(value, blinks)) {
        return count;
    }

    let count = match value {
        0 => count_stones_rec(1, blinks - 1, memo),
        v if has_even_digit_count(v) => {
            let (l, r) = split_digits(v);
            count_stones_rec(l, blinks - 1, memo) + count_stones_rec(r, blinks - 1, memo)
        }
        v => count_stones_rec(v * 2024, blinks - 1, memo),
    };

    memo.insert((value, blinks), count);
    count
}

fn num_digits(v: u64) -> u32 {
    assert_ne!(v, 0);
    v.ilog10() + 1
}

fn split_digits(v: u64) -> (u64, u64) {
    let n = num_digits(v);
    assert!(n & 1 == 0);
    let h = n / 2;
    let s = 10u64.pow(h);
    let l = v / s;
    let r = v % s;
    (l, r)
}

fn has_even_digit_count(v: u64) -> bool {
    num_digits(v) & 1 == 0
}
