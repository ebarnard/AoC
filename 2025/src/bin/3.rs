use nom::{
    character::{char, satisfy},
    combinator::map,
    multi::{many1, separated_list0},
};

use aoc2025::nom::parse_all;

const TEST: &str = include_str!("../../inputs/3-test.txt");
const REAL: &str = include_str!("../../inputs/3-real.txt");

fn main() {
    assert_eq!(part1(TEST), 357);
    assert_eq!(part1(REAL), 17311);
    assert_eq!(part2(TEST), 3121910778619);
    assert_eq!(part2(REAL), 171419245422055);
}

fn parse(input: &str) -> Vec<Vec<u64>> {
    let bank = many1(map(satisfy(|c| c.is_numeric()), |c: char| {
        c.to_string().parse().unwrap()
    }));
    let banks = separated_list0(char::<_, ()>('\n'), bank);

    parse_all(input, banks)
}

fn part1(input: &str) -> u64 {
    let banks = parse(input);
    max_jolts(&banks, 2)
}

fn part2(input: &str) -> u64 {
    let banks = parse(input);
    max_jolts(&banks, 12)
}

fn max_jolts(banks: &Vec<Vec<u64>>, num_per_bank: usize) -> u64 {
    banks
        .iter()
        .map(|bank| {
            (1..=num_per_bank)
                .fold((0, 0), |(val, pos), j| {
                    let a = bank[pos..(bank.len() - num_per_bank + j)]
                        .iter()
                        .max()
                        .unwrap();
                    let i = bank[pos..].iter().position(|v| v == a).unwrap();
                    (val * 10 + a, pos + i + 1)
                })
                .0
        })
        .sum()
}
