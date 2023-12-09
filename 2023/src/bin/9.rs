const TEST: &str = include_str!("../../inputs/9-test.txt");
const REAL: &str = include_str!("../../inputs/9-real.txt");

fn main() {
    assert_eq!(part1(TEST), 114);
    assert_eq!(part1(REAL), 1581679977);
    assert_eq!(part2(TEST), 2);
    assert_eq!(part2(REAL), 889);
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect()
}

fn polynomial_difference_table(sequence: &[i32]) -> Vec<Vec<i32>> {
    let mut table = vec![sequence.to_vec()];
    while !table.last().unwrap().iter().all(|&d| d == 0) {
        let row = table
            .last()
            .unwrap()
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        table.push(row);
    }
    table
}

fn part1(input: &str) -> i32 {
    let sequences = parse(input);

    sequences
        .iter()
        .map(|sequence| {
            let table = polynomial_difference_table(sequence);
            table
                .iter()
                .rev()
                .fold(0, |value, row| row.last().unwrap() + value)
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let sequences = parse(input);

    sequences
        .iter()
        .map(|sequence| {
            let table = polynomial_difference_table(sequence);
            table.iter().rev().fold(0, |value, row| row[0] - value)
        })
        .sum()
}
